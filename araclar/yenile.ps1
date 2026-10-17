# Yeni derleme: çalışan CanMeter'i kenara alır (çalışan exe silinemez ama adı değişir), release derler.
# Çalışan uygulama (0.6.7+) yeni canmeter-app.exe'yi görünce savaş dışında kendini yeniden başlatır (Ctrl+Shift+R hemen).
# Eski kenara alınmış kopyalardan silinebilenler silinir. -Baslat: çalışan yoksa yeni exe'yi açar.
param([switch]$Baslat)
$kok = Split-Path -Parent $PSScriptRoot
$klasor = Join-Path $kok "target\release"
$exe = Join-Path $klasor "canmeter-app.exe"
Get-ChildItem $klasor -Filter "canmeter-app-calisan-*.exe" -ErrorAction SilentlyContinue | ForEach-Object {
    try { Remove-Item $_.FullName -ErrorAction Stop } catch { }
}
if (Test-Path $exe) {
    $n = 1
    while (Test-Path (Join-Path $klasor "canmeter-app-calisan-$n.exe")) { $n++ }
    Rename-Item $exe "canmeter-app-calisan-$n.exe"
}
Set-Location $kok
cargo build --release -p canmeter-app
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Output ("derlendi: " + $exe)
if ($Baslat -and -not (Get-Process canmeter-app -ErrorAction SilentlyContinue)) { Start-Process $exe }
