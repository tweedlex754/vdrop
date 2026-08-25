<#
.SYNOPSIS
    VDrop kurulum dosyasini GitHub Releases'ten indirir ve (istege bagli) calistirir.

.DESCRIPTION
    Releases sayfasindaki en son surumu bulur, x64 NSIS kurulum dosyasini
    (VDrop_<surum>_x64-setup.exe) indirir. Installer'i uretenin release.yml
    oldugunu unutmayin: hicbir v* etiketi itilmemisse Releases bostur ve bu
    script bunu acikca soyler, sessizce basarisiz olmaz.

.PARAMETER Tag
    Belirli bir surum etiketi (ornek: v0.1.0). Verilmezse en son surum alinir.

.PARAMETER Token
    Repo private ise GitHub personal access token. Kimliksiz istek private
    repoda 404 dondugu icin bu olmadan bulunamaz.

.PARAMETER OutDir
    Indirilen dosyanin klasoru. Varsayilan: Indirilenler.

.PARAMETER Msi
    NSIS .exe yerine .msi paketini indirir (kurumsal dagitim icin).

.PARAMETER Run
    Indirdikten sonra kurulumu baslatir.

.EXAMPLE
    .\Install-VDrop.ps1
    En son setup.exe'yi Indirilenler klasorune indirir.

.EXAMPLE
    .\Install-VDrop.ps1 -Tag v0.1.0 -Run
    v0.1.0 surumunu indirir ve kurulumu baslatir.

.EXAMPLE
    .\Install-VDrop.ps1 -Token $env:GH_TOKEN
    Private repodan indirir.
#>
[CmdletBinding()]
param(
    [string] $Repo   = 'tweedlex754/vdrop',
    [string] $Tag,
    [string] $Token,
    [string] $OutDir = (Join-Path $env:USERPROFILE 'Downloads'),
    [switch] $Msi,
    [switch] $Run
)

$ErrorActionPreference = 'Stop'
# Invoke-WebRequest'in ilerleme cubugu buyuk dosyalarda indirmeyi kat kat
# yavaslatir; kapatmak dakikalar kazandirir.
$ProgressPreference = 'SilentlyContinue'

# --- API istegi icin ortak baslikler ------------------------------------
$headers = @{
    'Accept'               = 'application/vnd.github+json'
    'X-GitHub-Api-Version' = '2022-11-28'
    'User-Agent'           = 'Install-VDrop'
}
if ($Token) { $headers['Authorization'] = "Bearer $Token" }

$api = if ($Tag) {
    "https://api.github.com/repos/$Repo/releases/tags/$Tag"
} else {
    "https://api.github.com/repos/$Repo/releases/latest"
}

Write-Host "Surum bilgisi aliniyor: $Repo" -ForegroundColor Cyan

try {
    $release = Invoke-RestMethod -Uri $api -Headers $headers
}
catch {
    $code = $_.Exception.Response.StatusCode.value__
    switch ($code) {
        404 {
            # throw cok satirli metni tek satira ezer; taniyi once basip
            # sonra kisa bir hata firlatmak okunabilirligi koruyor.
            Write-Host ''
            Write-Host 'Surum bulunamadi (HTTP 404). Uc olasilik var:' -ForegroundColor Red
            Write-Host '  1. Repo private -> -Token <PAT> ile tekrar deneyin.'
            Write-Host '  2. Hic v* etiketi itilmemis -> Releases bos. Bir surum uretmek icin:'
            Write-Host '         git tag v0.1.0' -ForegroundColor Cyan
            Write-Host '         git push origin v0.1.0' -ForegroundColor Cyan
            Write-Host '     (release.yml calisir ve installer''lari Releases''e yukler.)'
            Write-Host '  3. Repo adi yanlis -> -Repo <sahip>/<ad> ile duzeltin.'
            Write-Host ''
            throw "Surum bulunamadi: $Repo (HTTP 404)"
        }
        401 { throw 'Token gecersiz veya suresi dolmus (HTTP 401).' }
        403 { throw 'Erisim reddedildi ya da API limiti asildi (HTTP 403). Token kullanin.' }
        default { throw "GitHub API hatasi (HTTP $code): $($_.Exception.Message)" }
    }
}

# --- Dogru varligi (asset) sec ------------------------------------------
# Tauri hem NSIS hem MSI uretir; ayrica .sig imza dosyalari da birakir,
# onlari elemek icin uzanti tam eslesmeli.
$pattern = if ($Msi) { '*_x64*.msi' } else { '*_x64-setup.exe' }
$asset = $release.assets | Where-Object { $_.name -like $pattern } | Select-Object -First 1

if (-not $asset) {
    $mevcut = ($release.assets | ForEach-Object { $_.name }) -join ', '
    throw "Surum '$($release.tag_name)' icinde '$pattern' ile eslesen dosya yok. Mevcut dosyalar: $mevcut"
}

if (-not (Test-Path $OutDir)) { New-Item -ItemType Directory -Path $OutDir -Force | Out-Null }
$hedef = Join-Path $OutDir $asset.name
$mb    = [math]::Round($asset.size / 1MB, 1)

Write-Host "Surum : $($release.tag_name)"
Write-Host "Dosya : $($asset.name) ($mb MB)"
Write-Host "Hedef : $hedef"
Write-Host 'Indiriliyor...' -ForegroundColor Cyan

# Private repolarda tarayici_download_url dogrudan calismaz; asset'in API
# adresinden octet-stream isteyerek her iki durumda da ayni yol calisir.
$dlHeaders = $headers.Clone()
$dlHeaders['Accept'] = 'application/octet-stream'

Invoke-WebRequest -Uri $asset.url -Headers $dlHeaders -OutFile $hedef

$inen = (Get-Item $hedef).Length
if ($inen -ne $asset.size) {
    Remove-Item $hedef -Force
    throw "Indirme eksik: $inen bayt geldi, $($asset.size) bekleniyordu. Dosya silindi."
}

Write-Host "Tamam: $hedef" -ForegroundColor Green
Write-Host "SHA256: $((Get-FileHash $hedef -Algorithm SHA256).Hash)" -ForegroundColor DarkGray
Write-Host 'Not: Windows derlemeleri imzali degil; SmartScreen ilk calistirmada uyarir.' -ForegroundColor Yellow

if ($Run) {
    Write-Host 'Kurulum baslatiliyor...' -ForegroundColor Cyan
    Start-Process -FilePath $hedef -Wait
}
