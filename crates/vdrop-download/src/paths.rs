//! Dosya adi sanitizasyonu ve guvenli hedef yolu uretimi.
//!
//! `docs/ARCHITECTURE.md` bolum O bunu Faz 1'in bilinen acigi olarak
//! isaretlemisti. Tehdit modeli: `title` alani **uzaktan gelen, guvenilmeyen**
//! veridir (provider bir HTML sayfasinin <title>'indan ya da URL'in son
//! parcasindan okur). Sanitize edilmezse sunlar mumkun:
//!
//!   - `../../../../Windows/System32/drivers/etc/hosts`  -> path traversal
//!   - `C:\Windows\evil.exe`                             -> mutlak yola kacis
//!   - `CON`, `LPT1`, `NUL`                              -> Windows aygit adlari
//!   - `report.txt.` / `report.txt `                     -> Windows sondaki
//!     nokta/bosluku sessizce atar; dogruladiginiz ad ile diske yazilan ad
//!     farkli olur (TOCTOU sinifi bir uyusmazlik)
//!   - `evil\u{202E}gnp.exe`                             -> RTL-override ile
//!     kullaniciya "evilexe.png" gibi gorunen calistirilabilir dosya
//!
//! Strateji: izin verilenler listesi degil, **agresif reddetme** + son savunma
//! hatti olarak `safe_join`'in uretilen yolun gercekten hedef klasorun icinde
//! kaldigini dogrulamasi.

use std::path::{Component, Path, PathBuf};

/// Windows'ta ayrilmis aygit adlari. Uzantiyla birlikte bile ayrilmistir
/// ("CON.txt" da acilamaz), bu yuzden govdeye bakariz.
const RESERVED: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// Tum platformlarda yasaklanan karakterler. Windows'un kumesi en dar oldugu
/// icin onu baz aliyoruz; boylece ayni ad her isletim sisteminde gecerli olur.
const ILLEGAL: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

/// Uzanti korunarak toplam ad uzunlugu siniri (bayt). Cogu dosya sistemi 255
/// bayt; ".part" eki ve "(2)" son eki icin pay birakiyoruz.
const MAX_LEN: usize = 200;

/// Guvenilmeyen bir metni tek bir dosya adi bilesenine indirger. Donen deger
/// asla yol ayraci, `..`, ayrilmis aygit adi veya bos dize olmaz.
pub fn sanitize_filename(input: &str) -> String {
    // 1) Yol bilesenlerini at: sadece son parcayi al ("a/b/c.mp4" -> "c.mp4").
    //    Hem '/' hem '\' bakiyoruz; Linux'ta '\' gecerli bir ad karakteridir
    //    ama biz Windows'a tasinabilirlik icin onu da ayrac sayiyoruz.
    let last = input
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(input);

    // 2) Kontrol karakterleri, yasakli karakterler ve iki yonlu metin (bidi)
    //    override'larini etkisizlestir.
    let mut cleaned: String = last
        .chars()
        .map(|c| {
            if c.is_control() || ILLEGAL.contains(&c) || is_bidi_override(c) {
                '_'
            } else {
                c
            }
        })
        .collect();

    // 3) Bosluklari sadelestir.
    cleaned = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");

    // 4) Windows sondaki nokta ve bosluklari sessizce atar; biz atalim ki
    //    dogruladigimiz ad ile yazilan ad ayni olsun.
    let cleaned = cleaned
        .trim_matches(|c: char| c == '.' || c == ' ')
        .to_string();

    // 5) Tamamen eridiyse yedek ada dus.
    if cleaned.is_empty() {
        return "vdrop-download".to_string();
    }

    // 6) Ayrilmis aygit adlarini etkisizlestir (uzantidan bagimsiz).
    let stem = cleaned.split('.').next().unwrap_or(&cleaned);
    let cleaned = if RESERVED.iter().any(|r| r.eq_ignore_ascii_case(stem)) {
        format!("_{cleaned}")
    } else {
        cleaned
    };

    truncate_preserving_extension(&cleaned, MAX_LEN)
}

/// U+202A..U+202E ve U+2066..U+2069: gorsel olarak dosya adini tersine
/// cevirip gercek uzantiyi gizleyebilen karakterler.
fn is_bidi_override(c: char) -> bool {
    matches!(c, '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}' | '\u{200E}' | '\u{200F}')
}

/// Uzunluk sinirini uygularken uzantiyi korur: cok uzun bir ad kesilince yine
/// `.mp4` ile bitmelidir, yoksa dosya iliskilendirmesi bozulur.
fn truncate_preserving_extension(name: &str, max: usize) -> String {
    if name.len() <= max {
        return name.to_string();
    }
    let (stem, ext) = match name.rfind('.') {
        // Uzanti makul uzunlukta mi? "1.2.3-surum-notlari" gibi adlarda son
        // noktadan sonrasi uzanti degildir.
        Some(i) if name.len() - i <= 12 && i > 0 => (&name[..i], &name[i..]),
        _ => (name, ""),
    };
    let budget = max.saturating_sub(ext.len());
    let mut cut = budget.min(stem.len());
    // UTF-8 karakter sinirina hizala; ortadan kesip gecersiz dizi uretmeyelim.
    while cut > 0 && !stem.is_char_boundary(cut) {
        cut -= 1;
    }
    format!("{}{}", &stem[..cut], ext)
}

#[derive(Debug, thiserror::Error)]
pub enum PathError {
    #[error("hedef klasor yolu gecersiz: {0}")]
    InvalidFolder(String),
    #[error("uretilen yol hedef klasorun disina cikiyor")]
    Escape,
}

/// Bir klasor + guvenilmeyen dosya adindan guvenli bir tam yol uretir.
///
/// Iki katmanli savunma: adi sanitize eder **ve** sonucun gercekten `folder`
/// klasorunun dogrudan cocugu oldugunu dogrular. Ikinci kontrol, ilkinde
/// gozden kacan bir sey olursa yakalar.
pub fn safe_join(folder: &Path, untrusted_name: &str) -> Result<PathBuf, PathError> {
    if folder.as_os_str().is_empty() {
        return Err(PathError::InvalidFolder(folder.display().to_string()));
    }
    let name = sanitize_filename(untrusted_name);
    let candidate = folder.join(&name);

    // folder'in bilesen sayisindan sonrasinda tam olarak bir "normal" bilesen
    // kalmali; `..`, kok ya da `C:\` gibi bir prefix bileseni kalmamali.
    let extra: Vec<Component> = candidate
        .components()
        .skip(folder.components().count())
        .collect();
    let escapes = extra.len() != 1 || !matches!(extra.first(), Some(Component::Normal(_)));
    if escapes {
        return Err(PathError::Escape);
    }
    Ok(candidate)
}

/// Ayni adda dosya varsa "ad (2).mp4", "ad (3).mp4" ... uretir. Kullanicinin
/// onceki indirmesini sessizce ezmemek icin.
pub fn unique_destination(path: &Path) -> PathBuf {
    unique_destination_with(path, |_| false)
}

/// `unique_destination`in, dosya sisteminde **henuz gorunmeyen** adlari da
/// hesaba katan hali.
///
/// Neden gerekli: bir indirme olusturuldugunda hedef adi secilir ama `.part`
/// dosyasi ancak transfer basladiginda yaratilir. Es zamanlilik kuyrugunda
/// bekleyen bir indirme icin bu ikisi arasinda uzun bir sure gecebilir. O
/// aralikta ayni adi isteyen ikinci bir indirme olusturulursa, dosya sistemi
/// "bos" dedigi icin **ayni adi** alir ve ikisi de ayni `.part` dosyasina
/// yazar - cikti bozulur.
///
/// `taken` yuklemi, cagiran tarafin kendi bekleyen kayitlarini bildirmesini
/// saglar (VDrop'ta: veritabanindaki sonlanmamis indirmeler).
pub fn unique_destination_with(path: &Path, taken: impl Fn(&Path) -> bool) -> PathBuf {
    let is_free = |p: &Path| !p.exists() && !part_sibling_exists(p) && !taken(p);

    if is_free(path) {
        return path.to_path_buf();
    }
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let full = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let (stem, ext) = match full.rfind('.') {
        Some(i) if i > 0 => (full[..i].to_string(), full[i..].to_string()),
        _ => (full.clone(), String::new()),
    };
    for n in 2..10_000u32 {
        let candidate = parent.join(format!("{stem} ({n}){ext}"));
        if is_free(&candidate) {
            return candidate;
        }
    }
    path.to_path_buf()
}

/// Yarim kalmis bir `.part` dosyasi da "bu ad kullanimda" sayilir; yoksa
/// duraklatilmis bir indirmenin uzerine ikinci bir indirme yazabilir.
fn part_sibling_exists(path: &Path) -> bool {
    let mut p = path.to_path_buf();
    let name = p.file_name().map(|n| format!("{}.part", n.to_string_lossy()));
    match name {
        Some(n) => {
            p.set_file_name(n);
            p.exists()
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_path_traversal() {
        assert_eq!(sanitize_filename("../../etc/passwd"), "passwd");
        assert_eq!(
            sanitize_filename("..\\..\\Windows\\System32\\evil.exe"),
            "evil.exe"
        );
        assert_eq!(sanitize_filename("/absolute/path/video.mp4"), "video.mp4");
        assert_eq!(sanitize_filename("C:\\Windows\\evil.exe"), "evil.exe");
    }

    #[test]
    fn dotdot_alone_becomes_fallback() {
        assert_eq!(sanitize_filename(".."), "vdrop-download");
        assert_eq!(sanitize_filename("."), "vdrop-download");
        assert_eq!(sanitize_filename("   "), "vdrop-download");
        assert_eq!(sanitize_filename(""), "vdrop-download");
    }

    #[test]
    fn replaces_illegal_characters() {
        assert_eq!(
            sanitize_filename("a<b>c:d\"e|f?g*h.mp4"),
            "a_b_c_d_e_f_g_h.mp4"
        );
    }

    #[test]
    fn neutralizes_windows_reserved_names() {
        assert_eq!(sanitize_filename("CON"), "_CON");
        assert_eq!(sanitize_filename("nul.txt"), "_nul.txt");
        assert_eq!(sanitize_filename("COM9.mp4"), "_COM9.mp4");
        // Ayrilmis olmayan benzer adlar dokunulmadan gecmeli.
        assert_eq!(sanitize_filename("CONCERT.mp4"), "CONCERT.mp4");
    }

    #[test]
    fn trims_trailing_dots_and_spaces() {
        assert_eq!(sanitize_filename("report.txt."), "report.txt");
        assert_eq!(sanitize_filename("report.txt   "), "report.txt");
    }

    #[test]
    fn strips_bidi_override_used_to_fake_extensions() {
        // "evil<RLO>gnp.exe" kullaniciya "evilexe.png" gibi gorunur.
        let faked = "evil\u{202E}gnp.exe";
        let out = sanitize_filename(faked);
        assert!(!out.contains('\u{202E}'), "bidi override kalmamali: {out:?}");
        assert!(out.ends_with(".exe"), "gercek uzanti gorunur kalmali: {out:?}");
    }

    #[test]
    fn control_characters_are_replaced() {
        let out = sanitize_filename("video\u{0007}\u{0000}.mp4");
        assert_eq!(out, "video__.mp4");
    }

    #[test]
    fn truncates_but_keeps_extension() {
        let long = format!("{}.mp4", "u".repeat(500));
        let out = sanitize_filename(&long);
        assert!(out.len() <= MAX_LEN, "sinir asildi: {}", out.len());
        assert!(out.ends_with(".mp4"), "uzanti korunmali: {out:?}");
    }

    #[test]
    fn truncation_never_splits_a_utf8_char() {
        // Cok baytli karakterlerden olusan uzun ad: kesme noktasi karakter
        // ortasina denk gelirse gecersiz UTF-8 uretilirdi.
        let long = format!("{}.mp4", "ç".repeat(400));
        let out = sanitize_filename(&long);
        assert!(out.len() <= MAX_LEN);
        assert!(out.ends_with(".mp4"));
        assert!(out.chars().count() > 0);
    }

    #[test]
    fn safe_join_keeps_file_inside_folder() {
        let folder = Path::new("/downloads");
        let p = safe_join(folder, "../../etc/passwd").unwrap();
        assert_eq!(p.parent().unwrap(), folder);
        assert_eq!(p.file_name().unwrap(), "passwd");
    }

    #[test]
    fn safe_join_rejects_empty_folder() {
        assert!(matches!(
            safe_join(Path::new(""), "x.mp4"),
            Err(PathError::InvalidFolder(_))
        ));
    }

    /// KANIT: ad rezervasyonu dosya sistemine bagli oldugu icin, dosya
    /// henuz olusmadan iki kez sorulursa AYNI adi verir.
    ///
    /// Gercek senaryo: kullanici bir sayfadan once 480p sonra 240p indiriyor.
    /// Ikisinin de basligi ayni, yani onerilen dosya adi ayni. Ilki es
    /// zamanlilik kuyrugunda beklerken (henuz `.part` olusturmadan) ikincisi
    /// olusturulursa, ikisi de ayni `.part` dosyasina yazar ve cikti bozulur.
    #[test]
    fn unique_destination_alone_cannot_reserve_a_name() {
        let dir = std::env::temp_dir().join(format!("vdrop-race-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        let target = dir.join("clip.mp4");

        let first = unique_destination(&target);
        let second = unique_destination(&target);
        assert_eq!(
            first, second,
            "dosya sistemi tek basina rezervasyon yapamaz; cagiran taraf              bekleyen indirmeleri de hesaba katmali"
        );

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn unique_destination_with_extra_taken_predicate_reserves_across_pending() {
        let dir = std::env::temp_dir().join(format!("vdrop-race2-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        let target = dir.join("clip.mp4");

        // Cagiran taraf, veritabanindaki bekleyen kayitlari bildirir.
        let mut reserved: Vec<PathBuf> = Vec::new();

        let first = unique_destination_with(&target, |p| reserved.iter().any(|r| r == p));
        reserved.push(first.clone());
        let second = unique_destination_with(&target, |p| reserved.iter().any(|r| r == p));
        reserved.push(second.clone());
        let third = unique_destination_with(&target, |p| reserved.iter().any(|r| r == p));

        assert_eq!(first.file_name().unwrap(), "clip.mp4");
        assert_eq!(second.file_name().unwrap(), "clip (2).mp4");
        assert_eq!(third.file_name().unwrap(), "clip (3).mp4");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn unique_destination_avoids_overwrite() {
        let dir = std::env::temp_dir().join(format!("vdrop-uniq-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        let target = dir.join("clip.mp4");
        assert_eq!(unique_destination(&target), target); // henuz yok

        std::fs::write(&target, b"x").unwrap();
        assert_eq!(unique_destination(&target), dir.join("clip (2).mp4"));

        // Yarim kalmis .part dosyasi da adi rezerve eder.
        std::fs::write(dir.join("clip (2).mp4.part"), b"x").unwrap();
        assert_eq!(unique_destination(&target), dir.join("clip (3).mp4"));

        std::fs::remove_dir_all(&dir).ok();
    }
}
