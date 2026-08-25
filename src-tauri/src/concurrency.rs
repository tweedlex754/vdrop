//! Es zamanli indirme siniri.
//!
//! Kullanici "ayni anda en fazla 3 indirme" dediginde, dorduncu indirme
//! **iptal edilmemeli**, sirasini beklemelidir. Bunu bir Tokio semaforu ile
//! yapiyoruz: her indirme gorevi transfere baslamadan once bir izin alir.
//!
//! Isin puf noktasi limitin **calisma sirasinda degistirilebilmesi**:
//!
//! - Limit artarsa: `add_permits` ile bekleyenlerden biri hemen baslar.
//! - Limit azalirsa: calisan indirmeleri oldurmek yanlis olur (kullanici
//!   veriyi kaybeder). Bunun yerine fazla izinleri "emiyoruz" - izinler
//!   ancak mevcut indirmeler bitince serbest kalir ve geri verilmez.
//!   Yani yeni limit **kademeli** olarak yururluge girer.

use std::sync::Arc;

use tokio::sync::{Mutex, OwnedSemaphorePermit, Semaphore};

/// Semafor izin sayisinin ust siniri. Tokio'nun kendi siniri cok daha yuksek
/// ama makul bir tavan, hatali bir ayar degerinin binlerce es zamanli soket
/// acmasini engeller.
pub const MAX_CONCURRENT: usize = 16;

pub struct ConcurrencyGate {
    sem: Arc<Semaphore>,
    /// Hedeflenen limit. `sem`'in anlik izin sayisi bundan gecici olarak
    /// farkli olabilir (limit dusurulup indirmeler henuz bitmediyse).
    target: Mutex<usize>,
}

impl ConcurrencyGate {
    pub fn new(limit: usize) -> Self {
        let limit = limit.clamp(1, MAX_CONCURRENT);
        Self {
            sem: Arc::new(Semaphore::new(limit)),
            target: Mutex::new(limit),
        }
    }

    /// Transfer baslamadan once cagrilir. Slot bosalana kadar bekler.
    /// Donen izin dusurulunce (drop) slot otomatik serbest kalir.
    pub async fn acquire(&self) -> OwnedSemaphorePermit {
        self.sem
            .clone()
            .acquire_owned()
            .await
            .expect("semafor kapatilmadi")
    }

    pub async fn limit(&self) -> usize {
        *self.target.lock().await
    }

    /// Limiti calisma sirasinda degistirir.
    pub async fn set_limit(&self, new_limit: usize) {
        let new_limit = new_limit.clamp(1, MAX_CONCURRENT);
        let mut target = self.target.lock().await;
        let old = *target;
        if new_limit == old {
            return;
        }
        *target = new_limit;

        if new_limit > old {
            self.sem.add_permits(new_limit - old);
        } else {
            // Fazla izinleri kalici olarak yut. `acquire_many` bekleyebilir
            // (calisan indirmeler bitene kadar), o yuzden arka plana aliyoruz;
            // ayarlar ekrani kilitlenmesin.
            let sem = self.sem.clone();
            let surplus = (old - new_limit) as u32;
            tokio::spawn(async move {
                if let Ok(permit) = sem.acquire_many_owned(surplus).await {
                    permit.forget();
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    #[tokio::test]
    async fn limits_simultaneous_holders() {
        let gate = Arc::new(ConcurrencyGate::new(2));
        let live = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));

        let mut tasks = Vec::new();
        for _ in 0..8 {
            let gate = gate.clone();
            let live = live.clone();
            let peak = peak.clone();
            tasks.push(tokio::spawn(async move {
                let permit = gate.acquire().await;
                let now = live.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(now, Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(20)).await;
                live.fetch_sub(1, Ordering::SeqCst);
                drop(permit);
            }));
        }
        for t in tasks {
            t.await.unwrap();
        }
        assert_eq!(peak.load(Ordering::SeqCst), 2, "limit asilmamali");
    }

    #[tokio::test]
    async fn raising_limit_releases_waiters_immediately() {
        let gate = Arc::new(ConcurrencyGate::new(1));
        let _held = gate.acquire().await; // tek izni tut

        // Limit 1 iken ikinci acquire bloke olmali.
        let gate2 = gate.clone();
        let waiter = tokio::spawn(async move { gate2.acquire().await });
        tokio::time::sleep(Duration::from_millis(30)).await;
        assert!(!waiter.is_finished(), "limit 1 iken beklemeliydi");

        gate.set_limit(3).await;
        let got = tokio::time::timeout(Duration::from_millis(500), waiter).await;
        assert!(got.is_ok(), "limit artinca bekleyen hemen baslamali");
        assert_eq!(gate.limit().await, 3);
    }

    #[tokio::test]
    async fn lowering_limit_never_interrupts_running_work() {
        let gate = Arc::new(ConcurrencyGate::new(4));
        let a = gate.acquire().await;
        let b = gate.acquire().await;

        gate.set_limit(1).await;
        assert_eq!(gate.limit().await, 1);

        // Calisan iki is hala izinlerini tutuyor: dusurme onlari kesmedi.
        drop(a);
        drop(b);

        // Yeni limit yururluge girdikten sonra ayni anda sadece 1 is gecmeli.
        tokio::time::sleep(Duration::from_millis(50)).await;
        let _first = gate.acquire().await;
        let gate2 = gate.clone();
        let second = tokio::spawn(async move { gate2.acquire().await });
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(!second.is_finished(), "dusurulen limit sonunda yururlukte olmali");
    }

    #[tokio::test]
    async fn limit_is_clamped_to_sane_range() {
        let gate = ConcurrencyGate::new(0);
        assert_eq!(gate.limit().await, 1, "0 anlamsiz, en az 1 olmali");

        let gate = ConcurrencyGate::new(9999);
        assert_eq!(gate.limit().await, MAX_CONCURRENT);
    }
}
