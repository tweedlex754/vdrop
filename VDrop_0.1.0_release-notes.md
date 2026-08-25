## VDrop 0.1.0

İlk sürüm. Bağlantıyı yapıştır, arkasında ne varsa listelensin, hangisini
indireceğini sen seç.

### İndir

| Dosya | Platform | Boyut |
|---|---|---|
| `VDrop_0.1.0_x64-setup.exe` | Windows 10/11 (x64) | 4.76 MB |

### Doğrulama

`VDrop_0.1.0_checksums.txt` içindeki SHA256 ile karşılaştır:

```powershell
(Get-FileHash .\VDrop_0.1.0_x64-setup.exe -Algorithm SHA256).Hash
```

### Bilinmesi gerekenler

- **Windows derlemesi imzalı değil.** SmartScreen ilk çalıştırmada uyarır:
  *Daha fazla bilgi → Yine de çalıştır.*
- **yt-dlp ve FFmpeg dahil değil.** İkisi de opsiyonel ve ayrı lisanslara
  tabi. Onlarsız doğrudan medya bağlantıları ve HLS/DASH manifestoları yine
  iner; yt-dlp site-özel çıkarma, FFmpeg segment birleştirme ekler.

  ```powershell
  pip install -U yt-dlp
  winget install ffmpeg
  ```

  Ayarlar → Bileşenler, hangisinin bulunduğunu gösterir.
- **DRM korumalı içerik desteklenmez** ve desteklenmeyecek. Sağlayıcı DRM
  bildirirse VDrop reddeder.

### Kapsam

Bu sürüm yalnızca Windows x64 içerir. macOS derlemeleri `release.yml`
üzerinden bir `v*` etiketi itildiğinde üretilir.
