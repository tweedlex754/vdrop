//! Paylasilan bant genisligi siniri (jeton kovasi).
//!
//! Neden **paylasilan**: kullanici "indirmelerim 500 KB/sn'yi gecmesin" der,
//! "her indirme ayri ayri 500 KB/sn olsun" demez. Uc indirme paralel
//! kosarken hedef toplam hizdir, o yuzden tek kova hepsini besler.
//!
//! Neden jeton kovasi ve uyku: akisi parca parca okuyoruz zaten; her parca
//! icin bedelini odeyip gerekirse bekliyoruz. TCP akis kontrolu gerisini
//! kendisi halleder - biz okumayi yavaslatinca pencere daralir ve sunucu
//! yavaslar.

use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

struct Bucket {
    /// Harcanabilir bayt. **Negatif olabilir**: bir parca kovadan fazlasini
    /// isterse borca giriyoruz ve borcu uyuyarak oduyoruz. Alternatif -
    /// dongude tekrar tekrar denemek - ayni parcayi iki kez ucretlendirme
    /// riskini tasiyordu.
    tokens: f64,
    last: Instant,
}

pub struct RateLimiter {
    /// Bayt/saniye. **0 = sinirsiz.** Calisma sirasinda degistirilebilir;
    /// es zamanlilik limiti de boyle davraniyor.
    rate: AtomicU64,
    bucket: Mutex<Bucket>,
}

impl RateLimiter {
    pub fn new(bytes_per_sec: u64) -> Self {
        Self {
            rate: AtomicU64::new(bytes_per_sec),
            bucket: Mutex::new(Bucket {
                // Kova dolu baslar: kisa bir dosya icin bir saniyelik patlama
                // serbest. Sifirdan baslasaydi her indirme ilk baytini
                // beklemek zorunda kalirdi.
                tokens: bytes_per_sec as f64,
                last: Instant::now(),
            }),
        }
    }

    /// Sinirsiz bir limitleyici. Ayar kapaliyken bunu kullaniyoruz ki
    /// cagri yerlerinde `Option` kontrolu dagilmasin.
    pub fn unlimited() -> Self {
        Self::new(0)
    }

    pub fn set_rate(&self, bytes_per_sec: u64) {
        self.rate.store(bytes_per_sec, Ordering::Relaxed);
    }

    pub fn rate(&self) -> u64 {
        self.rate.load(Ordering::Relaxed)
    }

    /// `bytes` kadar veri okumanin bedelini oder; gerekirse bekler.
    pub async fn acquire(&self, bytes: usize) {
        let rate = self.rate.load(Ordering::Relaxed);
        if rate == 0 || bytes == 0 {
            return;
        }

        let wait = {
            let mut bucket = self.bucket.lock().await;
            let now = Instant::now();
            let elapsed = now.saturating_duration_since(bucket.last).as_secs_f64();
            bucket.last = now;

            // Bir saniyelik tavan: uzun bir duraklamadan sonra kova
            // sinirsizca dolup limiti anlamsiz kilmasin.
            let ceiling = rate as f64;
            bucket.tokens = (bucket.tokens + elapsed * ceiling).min(ceiling);
            bucket.tokens -= bytes as f64;

            if bucket.tokens < 0.0 {
                Some(Duration::from_secs_f64(-bucket.tokens / ceiling))
            } else {
                None
            }
        };

        if let Some(delay) = wait {
            tokio::time::sleep(delay).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn unlimited_never_waits() {
        let limiter = RateLimiter::unlimited();
        let start = Instant::now();
        for _ in 0..1000 {
            limiter.acquire(1_000_000).await;
        }
        assert_eq!(start.elapsed(), Duration::ZERO);
    }

    #[tokio::test(start_paused = true)]
    async fn spends_the_full_bucket_before_waiting() {
        // Kova dolu basladigi icin ilk bir saniyelik veri bedavadir.
        let limiter = RateLimiter::new(1_000);
        let start = Instant::now();
        limiter.acquire(1_000).await;
        assert_eq!(start.elapsed(), Duration::ZERO, "ilk patlama beklememeli");
    }

    #[tokio::test(start_paused = true)]
    async fn holds_the_average_to_the_configured_rate() {
        let limiter = RateLimiter::new(1_000);
        let start = Instant::now();

        // 5.000 bayt: 1.000'i dolu kovadan, kalan 4.000 icin 4 saniye.
        for _ in 0..5 {
            limiter.acquire(1_000).await;
        }

        let elapsed = start.elapsed();
        assert_eq!(
            elapsed,
            Duration::from_secs(4),
            "5.000 bayt / 1.000 bayt-sn = 4 sn beklenirdi (ilk kova bedava)"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_chunk_larger_than_the_bucket_still_passes() {
        // Parca boyutu bizim elimizde degil: sunucu 256 KB gonderebilir,
        // limit 10 KB/sn olabilir. Boyle bir parca kilitlenmemeli, sadece
        // bedelini odemeli.
        let limiter = RateLimiter::new(1_000);
        let start = Instant::now();
        limiter.acquire(10_000).await;
        assert_eq!(start.elapsed(), Duration::from_secs(9));
    }

    #[tokio::test(start_paused = true)]
    async fn rate_changes_apply_to_the_next_chunk() {
        // Ayarlar ekraninda limiti degistiren kullanici, calisan
        // indirmelerin yeniden baslamasini beklememeli.
        let limiter = RateLimiter::new(1_000);
        limiter.acquire(1_000).await;

        limiter.set_rate(0);
        let start = Instant::now();
        limiter.acquire(1_000_000).await;
        assert_eq!(start.elapsed(), Duration::ZERO, "sinirsiza gecis aninda olmali");
    }

    #[tokio::test(start_paused = true)]
    async fn idle_time_does_not_bank_unlimited_credit() {
        // Uzun bir bosluktan sonra kova en fazla bir saniyelik dolar.
        let limiter = RateLimiter::new(1_000);
        limiter.acquire(1_000).await;
        tokio::time::sleep(Duration::from_secs(60)).await;

        let start = Instant::now();
        limiter.acquire(3_000).await;
        assert_eq!(
            start.elapsed(),
            Duration::from_secs(2),
            "60 saniye bosta beklemek 60 saniyelik kredi biriktirmemeli"
        );
    }
}
