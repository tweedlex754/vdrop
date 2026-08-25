//! vdrop-storage: yerel kalicilik katmani.
//!
//! SQLite (bundled - sistemde kurulu olmasi gerekmez) uzerinde indirmeler,
//! kuyruk, gecmis, ayarlar, favoriler ve kutuphane. Ileri-yonlu (forward-only)
//! migration kosucusu `schema_migrations` tablosuyla izlenir: yeni bir surum
//! gerektiginde `MIGRATIONS` dizisine **yeni bir girdi eklenir**, mevcut
//! girdiler asla degistirilmez. Boylece eski kurulumlar da tutarli sekilde
//! ilerler.
//!
//! Sirlar (cerez, token) bilincli olarak burada modellenmez: bunlarin isletim
//! sisteminin guvenli kimlik deposuna gitmesi gerekir, duz SQLite sutununa
//! degil.

use std::collections::HashMap;

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("veritabani hatasi: {0}")]
    Db(#[from] rusqlite::Error),
}

pub struct Storage {
    conn: Connection,
}

/// Bir indirmenin kalici hali. Uygulama kapanip acildiginda liste buradan
/// yeniden kurulur - aksi halde kullanici tum kuyrugunu kaybeder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRecord {
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub destination_path: String,
    pub total_bytes: Option<i64>,
    pub downloaded_bytes: i64,
    pub status: String,
    pub provider_id: Option<String>,
    /// "http" (duz dosya) veya "stream" (HLS/DASH, FFmpeg ile).
    pub kind: String,
    pub error_message: Option<String>,
    pub thumbnail_url: Option<String>,
    /// HLS master playlist'teki program indeksi; duz indirmelerde `None`.
    pub variant_index: Option<i64>,
    /// yt-dlp format kimligi; diger indirme turlerinde `None`.
    pub format_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub status: String,
    pub destination_path: Option<String>,
    pub total_bytes: Option<i64>,
    pub completed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItem {
    pub id: String,
    pub title: Option<String>,
    pub file_path: String,
    pub duration_seconds: Option<f64>,
    pub resolution: Option<String>,
    pub codec: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub downloaded_at: String,
}

/// Yeni bir indirme kaydinin alanlari.
///
/// Yapi olarak veriliyor cunku sekiz konumsal argumanli bir cagrida iki
/// `Option<&str>`'in yerini karistirmak derleme hatasi vermez - sessizce
/// yanlis sutuna yazar. Alan adlari bunu imkansiz kilar.
#[derive(Debug, Clone, Default)]
pub struct NewDownload<'a> {
    pub id: &'a str,
    pub url: &'a str,
    pub title: Option<&'a str>,
    pub destination_path: &'a str,
    /// "http" veya "stream".
    pub kind: &'a str,
    pub thumbnail_url: Option<&'a str>,
    pub provider_id: Option<&'a str>,
    pub variant_index: Option<i64>,
    pub format_id: Option<&'a str>,
}

const MIGRATIONS: &[(&str, &str)] = &[
    (
        "0001_init",
        r#"
        CREATE TABLE IF NOT EXISTS schema_migrations (
            name TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS downloads (
            id TEXT PRIMARY KEY,
            url TEXT NOT NULL,
            title TEXT,
            destination_path TEXT NOT NULL,
            total_bytes INTEGER,
            downloaded_bytes INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'queued',
            provider_id TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS history (
            id TEXT PRIMARY KEY,
            download_id TEXT,
            url TEXT NOT NULL,
            title TEXT,
            status TEXT NOT NULL,
            completed_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS favorites (
            id TEXT PRIMARY KEY,
            url TEXT NOT NULL,
            title TEXT,
            kind TEXT NOT NULL DEFAULT 'media',
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS library_items (
            id TEXT PRIMARY KEY,
            title TEXT,
            file_path TEXT NOT NULL,
            duration_seconds REAL,
            resolution TEXT,
            codec TEXT,
            file_size_bytes INTEGER,
            downloaded_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS component_versions (
            component TEXT PRIMARY KEY,
            installed_version TEXT NOT NULL,
            channel TEXT NOT NULL DEFAULT 'stable',
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        "#,
    ),
    (
        // v2: HLS/DASH akislari duz HTTP indirmelerinden ayirt edilebilmeli,
        // hata mesaji kalici olmali (uygulama kapanip acilinca "neden basarisiz
        // oldu" bilgisi kaybolmasin) ve gecmis kayitlari dosya yolunu tutmali
        // ki kullanici gecmisten dosyayi acabilsin.
        "0002_stream_kind_and_error",
        r#"
        ALTER TABLE downloads ADD COLUMN kind TEXT NOT NULL DEFAULT 'http';
        ALTER TABLE downloads ADD COLUMN error_message TEXT;
        ALTER TABLE downloads ADD COLUMN thumbnail_url TEXT;

        ALTER TABLE history ADD COLUMN destination_path TEXT;
        ALTER TABLE history ADD COLUMN total_bytes INTEGER;

        CREATE INDEX IF NOT EXISTS idx_downloads_status ON downloads(status);
        CREATE INDEX IF NOT EXISTS idx_history_completed ON history(completed_at DESC);
        CREATE INDEX IF NOT EXISTS idx_library_downloaded ON library_items(downloaded_at DESC);
        "#,
    ),
    (
        // v3: HLS varyant secimi kalici olmali. Aksi halde duraklatilan ya da
        // uygulama kapandiktan sonra devam ettirilen bir akis, kullanicinin
        // sectigi 1080p yerine FFmpeg'in varsayilan varyantini indirirdi.
        "0003_variant_index",
        r#"
        ALTER TABLE downloads ADD COLUMN variant_index INTEGER;
        "#,
    ),
    (
        // v4: yt-dlp format kimligi bir METINDIR ("137", "hls-1080", "bestaudio")
        // ve mevcut `variant_index` tamsayi sutununa sigmaz. Kalici olmali:
        // duraklatilip devam ettirilen bir indirme, kullanicinin sectigi
        // formata geri donmeli.
        "0004_format_id",
        r#"
        ALTER TABLE downloads ADD COLUMN format_id TEXT;
        "#,
    ),
];

impl Storage {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, StorageError> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        let mut storage = Self { conn };
        storage.migrate()?;
        Ok(storage)
    }

    pub fn open_in_memory() -> Result<Self, StorageError> {
        let conn = Connection::open_in_memory()?;
        let mut storage = Self { conn };
        storage.migrate()?;
        Ok(storage)
    }

    fn migrate(&mut self) -> Result<(), StorageError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_migrations (name TEXT PRIMARY KEY, applied_at TEXT NOT NULL DEFAULT (datetime('now')));",
        )?;
        for (name, sql) in MIGRATIONS {
            let already: bool = self.conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE name = ?1)",
                [name],
                |r| r.get(0),
            )?;
            if !already {
                // Her migration tek bir islemde: yarida kalirsa hicbiri
                // uygulanmamis sayilir, sema tutarsiz kalmaz.
                let tx = self.conn.transaction()?;
                tx.execute_batch(sql)?;
                tx.execute("INSERT INTO schema_migrations(name) VALUES (?1)", [name])?;
                tx.commit()?;
            }
        }
        Ok(())
    }

    // ---- Ayarlar ---------------------------------------------------------

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT INTO settings(key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            (key, value),
        )?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, StorageError> {
        Ok(self
            .conn
            .query_row("SELECT value FROM settings WHERE key = ?1", [key], |r| {
                r.get(0)
            })
            .optional()?)
    }

    /// Tum ayarlari tek seferde okur. Arayuz acilista bunu bir kez cagirir;
    /// her ayar icin ayri IPC turu atmaktan cok daha ucuz.
    pub fn all_settings(&self) -> Result<HashMap<String, String>, StorageError> {
        let mut stmt = self.conn.prepare("SELECT key, value FROM settings")?;
        let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
        let mut map = HashMap::new();
        for row in rows {
            let (k, v) = row?;
            map.insert(k, v);
        }
        Ok(map)
    }

    // ---- Indirmeler ------------------------------------------------------

    pub fn insert_download(&self, new: &NewDownload<'_>) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT INTO downloads(id, url, title, destination_path, kind, thumbnail_url,
                                   provider_id, variant_index, format_id, status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'queued')",
            params![
                new.id,
                new.url,
                new.title,
                new.destination_path,
                new.kind,
                new.thumbnail_url,
                new.provider_id,
                new.variant_index,
                new.format_id,
            ],
        )?;
        Ok(())
    }

    pub fn update_progress(
        &self,
        id: &str,
        downloaded_bytes: i64,
        total_bytes: Option<i64>,
    ) -> Result<(), StorageError> {
        self.conn.execute(
            "UPDATE downloads
                SET downloaded_bytes = ?2,
                    total_bytes = COALESCE(?3, total_bytes),
                    updated_at = datetime('now')
              WHERE id = ?1",
            params![id, downloaded_bytes, total_bytes],
        )?;
        Ok(())
    }

    pub fn set_status(
        &self,
        id: &str,
        status: &str,
        error_message: Option<&str>,
    ) -> Result<(), StorageError> {
        self.conn.execute(
            "UPDATE downloads
                SET status = ?2, error_message = ?3, updated_at = datetime('now')
              WHERE id = ?1",
            params![id, status, error_message],
        )?;
        Ok(())
    }

    pub fn list_downloads(&self) -> Result<Vec<DownloadRecord>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, url, title, destination_path, total_bytes, downloaded_bytes, status,
                    provider_id, kind, error_message, thumbnail_url, variant_index, format_id,
                    created_at, updated_at
               FROM downloads
              ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([], row_to_download)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn get_download(&self, id: &str) -> Result<Option<DownloadRecord>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, url, title, destination_path, total_bytes, downloaded_bytes, status,
                    provider_id, kind, error_message, thumbnail_url, variant_index, format_id,
                    created_at, updated_at
               FROM downloads WHERE id = ?1",
        )?;
        Ok(stmt.query_row([id], row_to_download).optional()?)
    }

    pub fn delete_download(&self, id: &str) -> Result<(), StorageError> {
        self.conn
            .execute("DELETE FROM downloads WHERE id = ?1", [id])?;
        Ok(())
    }

    /// Biten/iptal edilen/basarisiz kayitlari listeden temizler; aktif olanlara
    /// dokunmaz.
    pub fn clear_finished_downloads(&self) -> Result<usize, StorageError> {
        Ok(self.conn.execute(
            "DELETE FROM downloads WHERE status IN ('completed','cancelled','failed')",
            [],
        )?)
    }

    /// Uygulama cokerse ya da kapatilirsa "downloading" durumunda kalmis
    /// kayitlar olur. Acilista bunlari "paused"a cekiyoruz: yalan soyleyen bir
    /// ilerleme cubugu yerine devam ettirilebilir bir durum gosterilir.
    pub fn reconcile_interrupted(&self) -> Result<usize, StorageError> {
        Ok(self.conn.execute(
            "UPDATE downloads SET status = 'paused', updated_at = datetime('now')
              WHERE status IN ('downloading','queued','retrying')",
            [],
        )?)
    }

    /// Bu hedef yol, sonlanmamis bir indirme tarafindan rezerve edilmis mi?
    ///
    /// Dosya sistemine bakmak yetmez: kuyrukta bekleyen bir indirmenin
    /// `.part` dosyasi henuz yaratilmamistir. O aralikta ayni adi isteyen
    /// ikinci bir indirme olusturulursa ikisi de ayni dosyaya yazar.
    pub fn destination_reserved(&self, path: &str) -> Result<bool, StorageError> {
        Ok(self.conn.query_row(
            "SELECT EXISTS(
                 SELECT 1 FROM downloads
                  WHERE destination_path = ?1
                    AND status NOT IN ('completed','cancelled','failed')
             )",
            [path],
            |r| r.get(0),
        )?)
    }

    pub fn count_downloads(&self) -> Result<i64, StorageError> {
        Ok(self
            .conn
            .query_row("SELECT COUNT(*) FROM downloads", [], |r| r.get(0))?)
    }

    // ---- Gecmis ----------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn add_history(
        &self,
        id: &str,
        download_id: Option<&str>,
        url: &str,
        title: Option<&str>,
        status: &str,
        destination_path: Option<&str>,
        total_bytes: Option<i64>,
    ) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT INTO history(id, download_id, url, title, status, destination_path, total_bytes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, download_id, url, title, status, destination_path, total_bytes],
        )?;
        Ok(())
    }

    pub fn list_history(&self, limit: i64) -> Result<Vec<HistoryRecord>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, url, title, status, destination_path, total_bytes, completed_at
               FROM history ORDER BY completed_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit], |r| {
            Ok(HistoryRecord {
                id: r.get(0)?,
                url: r.get(1)?,
                title: r.get(2)?,
                status: r.get(3)?,
                destination_path: r.get(4)?,
                total_bytes: r.get(5)?,
                completed_at: r.get(6)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn clear_history(&self) -> Result<usize, StorageError> {
        Ok(self.conn.execute("DELETE FROM history", [])?)
    }

    // ---- Kutuphane -------------------------------------------------------

    pub fn add_library_item(
        &self,
        id: &str,
        title: Option<&str>,
        file_path: &str,
        file_size_bytes: Option<i64>,
    ) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO library_items(id, title, file_path, file_size_bytes)
             VALUES (?1, ?2, ?3, ?4)",
            params![id, title, file_path, file_size_bytes],
        )?;
        Ok(())
    }

    pub fn list_library(&self) -> Result<Vec<LibraryItem>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, file_path, duration_seconds, resolution, codec,
                    file_size_bytes, downloaded_at
               FROM library_items ORDER BY downloaded_at DESC",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(LibraryItem {
                id: r.get(0)?,
                title: r.get(1)?,
                file_path: r.get(2)?,
                duration_seconds: r.get(3)?,
                resolution: r.get(4)?,
                codec: r.get(5)?,
                file_size_bytes: r.get(6)?,
                downloaded_at: r.get(7)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn delete_library_item(&self, id: &str) -> Result<(), StorageError> {
        self.conn
            .execute("DELETE FROM library_items WHERE id = ?1", [id])?;
        Ok(())
    }
}

fn row_to_download(r: &rusqlite::Row<'_>) -> rusqlite::Result<DownloadRecord> {
    Ok(DownloadRecord {
        id: r.get(0)?,
        url: r.get(1)?,
        title: r.get(2)?,
        destination_path: r.get(3)?,
        total_bytes: r.get(4)?,
        downloaded_bytes: r.get(5)?,
        status: r.get(6)?,
        provider_id: r.get(7)?,
        kind: r.get(8)?,
        error_message: r.get(9)?,
        thumbnail_url: r.get(10)?,
        variant_index: r.get(11)?,
        format_id: r.get(12)?,
        created_at: r.get(13)?,
        updated_at: r.get(14)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed(storage: &Storage, id: &str) {
        storage
            .insert_download(&NewDownload {
                id,
                url: "https://example.com/v.mp4",
                title: Some("Ornek"),
                destination_path: "/tmp/v.mp4",
                kind: "http",
                provider_id: Some("direct-http"),
                ..Default::default()
            })
            .unwrap();
    }

    #[test]
    fn migrates_and_roundtrips_settings() {
        let storage = Storage::open_in_memory().unwrap();
        storage.set_setting("theme", "light").unwrap();
        assert_eq!(
            storage.get_setting("theme").unwrap().as_deref(),
            Some("light")
        );
        assert_eq!(storage.get_setting("missing").unwrap(), None);
    }

    #[test]
    fn all_settings_returns_every_key() {
        let storage = Storage::open_in_memory().unwrap();
        storage.set_setting("theme", "dark").unwrap();
        storage.set_setting("language", "tr").unwrap();
        let all = storage.all_settings().unwrap();
        assert_eq!(all.get("theme").map(String::as_str), Some("dark"));
        assert_eq!(all.get("language").map(String::as_str), Some("tr"));
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn setting_upsert_overwrites() {
        let storage = Storage::open_in_memory().unwrap();
        storage.set_setting("theme", "light").unwrap();
        storage.set_setting("theme", "dark").unwrap();
        assert_eq!(storage.get_setting("theme").unwrap().as_deref(), Some("dark"));
    }

    #[test]
    fn inserts_and_lists_downloads() {
        let storage = Storage::open_in_memory().unwrap();
        seed(&storage, "dl-1");
        assert_eq!(storage.count_downloads().unwrap(), 1);

        let list = storage.list_downloads().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "dl-1");
        assert_eq!(list[0].status, "queued");
        assert_eq!(list[0].kind, "http");
        assert_eq!(list[0].downloaded_bytes, 0);
    }

    #[test]
    fn progress_and_status_updates_persist() {
        let storage = Storage::open_in_memory().unwrap();
        seed(&storage, "dl-1");

        storage.update_progress("dl-1", 512, Some(2048)).unwrap();
        let rec = storage.get_download("dl-1").unwrap().unwrap();
        assert_eq!(rec.downloaded_bytes, 512);
        assert_eq!(rec.total_bytes, Some(2048));

        // total_bytes None gecildiginde mevcut deger korunmali (COALESCE).
        storage.update_progress("dl-1", 1024, None).unwrap();
        let rec = storage.get_download("dl-1").unwrap().unwrap();
        assert_eq!(rec.downloaded_bytes, 1024);
        assert_eq!(rec.total_bytes, Some(2048), "bilinen toplam silinmemeli");

        storage
            .set_status("dl-1", "failed", Some("ag hatasi"))
            .unwrap();
        let rec = storage.get_download("dl-1").unwrap().unwrap();
        assert_eq!(rec.status, "failed");
        assert_eq!(rec.error_message.as_deref(), Some("ag hatasi"));
    }

    #[test]
    fn reconcile_marks_interrupted_downloads_as_paused() {
        let storage = Storage::open_in_memory().unwrap();
        seed(&storage, "dl-1");
        seed(&storage, "dl-2");
        storage.set_status("dl-1", "downloading", None).unwrap();
        storage.set_status("dl-2", "completed", None).unwrap();

        let touched = storage.reconcile_interrupted().unwrap();
        assert_eq!(touched, 1, "sadece yarim kalan kayit degismeli");
        assert_eq!(storage.get_download("dl-1").unwrap().unwrap().status, "paused");
        assert_eq!(
            storage.get_download("dl-2").unwrap().unwrap().status,
            "completed",
            "tamamlanmis indirme bozulmamali"
        );
    }

    #[test]
    fn destination_is_reserved_by_pending_downloads_only() {
        let storage = Storage::open_in_memory().unwrap();
        assert!(!storage.destination_reserved("/tmp/v.mp4").unwrap());

        seed(&storage, "bekleyen");
        assert!(
            storage.destination_reserved("/tmp/v.mp4").unwrap(),
            "kuyrukta bekleyen bir indirme adi rezerve etmeli"
        );

        // Sonlanmis bir indirme adi birakir: dosya artik diskte, dosya
        // sistemi kontrolu devreye girer.
        storage.set_status("bekleyen", "completed", None).unwrap();
        assert!(!storage.destination_reserved("/tmp/v.mp4").unwrap());
    }

    #[test]
    fn clear_finished_keeps_active_downloads() {
        let storage = Storage::open_in_memory().unwrap();
        seed(&storage, "done");
        seed(&storage, "active");
        storage.set_status("done", "completed", None).unwrap();
        storage.set_status("active", "downloading", None).unwrap();

        assert_eq!(storage.clear_finished_downloads().unwrap(), 1);
        let list = storage.list_downloads().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "active");
    }

    #[test]
    fn history_roundtrip_and_clear() {
        let storage = Storage::open_in_memory().unwrap();
        storage
            .add_history(
                "h1",
                Some("dl-1"),
                "https://example.com/a.mp4",
                Some("A"),
                "completed",
                Some("/tmp/a.mp4"),
                Some(999),
            )
            .unwrap();
        let list = storage.list_history(50).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].status, "completed");
        assert_eq!(list[0].total_bytes, Some(999));

        assert_eq!(storage.clear_history().unwrap(), 1);
        assert!(storage.list_history(50).unwrap().is_empty());
    }

    #[test]
    fn library_roundtrip() {
        let storage = Storage::open_in_memory().unwrap();
        storage
            .add_library_item("lib-1", Some("Video"), "/tmp/video.mp4", Some(1234))
            .unwrap();
        let items = storage.list_library().unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].file_path, "/tmp/video.mp4");

        storage.delete_library_item("lib-1").unwrap();
        assert!(storage.list_library().unwrap().is_empty());
    }

    #[test]
    fn migrations_are_idempotent_across_reopen() {
        // Ayni dosya iki kez acildiginda migration tekrar calismamali; v2'nin
        // ALTER TABLE ifadeleri ikinci kez calisirsa "duplicate column" hatasi
        // verirdi. Bu test o regresyonu yakalar.
        let dir = std::env::temp_dir().join(format!("vdrop-mig-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        let db = dir.join("t.sqlite3");

        {
            let s = Storage::open(&db).unwrap();
            s.set_setting("theme", "dark").unwrap();
        }
        {
            let s = Storage::open(&db).unwrap();
            assert_eq!(s.get_setting("theme").unwrap().as_deref(), Some("dark"));
        }
        std::fs::remove_dir_all(&dir).ok();
    }
}
