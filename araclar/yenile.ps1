# Yeni derleme + kurulu kopya (CC kuralı `CanMeterCC\kural-kurulu-kopya.md`): meter, bekçi ve raporu release derler,
# sonra $MeterKlasor'e (kurulum.ps1) yazar. Kurulu kopyadaki çalışan exe silinemez ama adı değişir: X.exe ->
# X-calisan-N.exe, yeni dosya asıl adla kopyalanır. Meter (0.6.7+) yeni dosyayı görünce savaş dışında kendini yeniden başlatır
# (Ctrl+Shift+R hemen); bekçi kendini yenilemez, görevi burada durdurulup yeniden başlatılır. Silinebilen eski
# -calisan-* kopyaları temizlenir. -Baslat: kurulu kopyada çalışan meter yoksa açar. -Kopyasiz: yalnız derler.
param([switch]$Baslat, [switch]$Kopyasiz)
. (Join-Path $PSScriptRoot "kurulum.ps1")
$kok = Split-Path -Parent $PSScriptRoot
$cikti = Join-Path $kok "target\release"
$gorev = "CanMeter oyunla baslat"
$adlar = @("canmeter-app", "canmeter-bekci", "canmeter-rapor")

function Temizle($klasor, $ad) {
    Get-ChildItem $klasor -Filter "$ad-calisan-*.exe" -ErrorAction SilentlyContinue | ForEach-Object {
        try { Remove-Item $_.FullName -ErrorAction Stop } catch { }
    }
}
function KenaraAl($klasor, $ad) {
    $yol = Join-Path $klasor "$ad.exe"
    if (-not (Test-Path $yol)) { return }
    $n = 1
    while (Test-Path (Join-Path $klasor "$ad-calisan-$n.exe")) { $n++ }
    Rename-Item $yol "$ad-calisan-$n.exe"
}
function Ayni($a, $b) {
    if (-not (Test-Path $a) -or -not (Test-Path $b)) { return $false }
    $x = Get-Item $a; $y = Get-Item $b
    return ($x.Length -eq $y.Length) -and ($x.LastWriteTimeUtc -eq $y.LastWriteTimeUtc)
}

# 1) Derleme. target'tan hiçbir şey çalışmaz (kural), kenara alma yok: kaynak değişmediyse cargo dokunmaz ve
#    kurulu kopya "ayni" diye atlanır, meter boşuna yeniden başlamaz. Eski -calisan-* kalıntıları temizlenir.
foreach ($ad in $adlar) { Temizle $cikti $ad }
Set-Location $kok
cargo build --release -p canmeter-app -p canmeter-bekci -p canmeter-rapor
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Output ("derlendi: " + $cikti)
if ($Kopyasiz) { exit 0 }

# 2) Kurulu kopya: boyut ve zaman damgası aynıysa atlanır (Copy-Item zaman damgasını korur).
New-Item -ItemType Directory -Force -Path $MeterKlasor | Out-Null
foreach ($ad in $adlar) {
    $kaynak = Join-Path $cikti "$ad.exe"
    $hedef = Join-Path $MeterKlasor "$ad.exe"
    Temizle $MeterKlasor $ad
    if (Ayni $kaynak $hedef) { Write-Output ("ayni: " + $hedef); continue }
    # Bekçi görevi yalnız kurulu bekçiye kayıtlıysa yeniden başlatılır; eski yola kayıtlıysa uyarı (kurucu yönetici ister).
    $gorevNesne = if ($ad -eq "canmeter-bekci") { Get-ScheduledTask -TaskName $gorev -ErrorAction SilentlyContinue } else { $null }
    $bekci = $gorevNesne -and ($gorevNesne.Actions[0].Execute -eq $hedef)
    if ($bekci) { Stop-ScheduledTask -TaskName $gorev -ErrorAction SilentlyContinue; Start-Sleep -Seconds 1 }
    KenaraAl $MeterKlasor $ad
    Copy-Item $kaynak $hedef
    Temizle $MeterKlasor $ad
    Write-Output ("kurulu kopya guncellendi: " + $hedef)
    if ($bekci) { Start-ScheduledTask -TaskName $gorev -ErrorAction SilentlyContinue }
    elseif ($gorevNesne) { Write-Warning ("gorev eski yola kayitli (" + $gorevNesne.Actions[0].Execute + "); yonetici PowerShell'de araclar\oyunla-baslat-kur.ps1 -Baslat") }
}
if ($Baslat -and -not (Get-Process "canmeter-app*" -ErrorAction SilentlyContinue)) {
    Start-Process (Join-Path $MeterKlasor "canmeter-app.exe") -WorkingDirectory $MeterKlasor
}
