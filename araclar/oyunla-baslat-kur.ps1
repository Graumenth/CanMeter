# "CanMeter oyunla baslat" zamanlanmış görevini kurar ya da kaldırır. Görev, oturum açılışında penceresiz
# canmeter-bekci.exe'yi yönetici yetkisiyle ve süresiz çalıştırır; bekçi oyunu (AION2.exe) görünce meter'ı açar,
# meter aynı yetkiyi alır (oyun odaktayken global kısayollar ancak yöneticiyle çalışır). Oyuna dokunulmaz.
# Yönetici PowerShell'de çalıştırılır. Yeniden çalıştırmak günceller (çalışan bekçi durdurulur, yeni exe alınır).
# -Baslat: kurduktan sonra görevi hemen başlatır. -Kaldir: görevi ve çalışan bekçiyi kaldırır.
# -Exe: bekçi yolu (varsayılan kurulu kopya $MeterKlasor\canmeter-bekci.exe, bkz. kurulum.ps1; target\release
# kullanılmaz, CC kuralı kural-kurulu-kopya.md). -Argumanlar: bekçiye geçer (örn. "--iz D:\bekci.log"; "--birak" oyun kapanınca meter'ı açık bırakır).
param([switch]$Baslat, [switch]$Kaldir, [string]$Exe = "", [string]$Argumanlar = "")
. (Join-Path $PSScriptRoot "kurulum.ps1")
$ad = "CanMeter oyunla baslat"
$yonetici = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
    [Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $yonetici) { Write-Error "Yonetici PowerShell gerekir (yuksek yetkili gorev ancak oyle kaydedilir)."; exit 1 }
Stop-ScheduledTask -TaskName $ad -ErrorAction SilentlyContinue
Get-Process -Name "canmeter-bekci" -ErrorAction SilentlyContinue | Stop-Process -Force
if ($Kaldir) {
    Unregister-ScheduledTask -TaskName $ad -Confirm:$false -ErrorAction SilentlyContinue
    Write-Output ("kaldirildi: " + $ad)
    exit 0
}
if (-not $Exe) { $Exe = Join-Path $MeterKlasor "canmeter-bekci.exe" }
if (-not (Test-Path $Exe)) { Write-Error ("bekci yok: " + $Exe + " (once araclar\yenile.ps1 calistir)"); exit 1 }
$kullanici = ([Security.Principal.WindowsIdentity]::GetCurrent()).Name
if ($Argumanlar) {
    $eylem = New-ScheduledTaskAction -Execute $Exe -Argument $Argumanlar -WorkingDirectory (Split-Path -Parent $Exe)
} else {
    $eylem = New-ScheduledTaskAction -Execute $Exe -WorkingDirectory (Split-Path -Parent $Exe)
}
$tetik = New-ScheduledTaskTrigger -AtLogOn -User $kullanici
$ayar = New-ScheduledTaskSettingsSet -Hidden -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries `
    -ExecutionTimeLimit ([TimeSpan]::Zero) -MultipleInstances IgnoreNew `
    -RestartCount 3 -RestartInterval (New-TimeSpan -Minutes 1)
$kimlik = New-ScheduledTaskPrincipal -UserId $kullanici -LogonType Interactive -RunLevel Highest
Register-ScheduledTask -TaskName $ad -Action $eylem -Trigger $tetik -Settings $ayar -Principal $kimlik `
    -Description "Oyun (AION2) acilinca CanMeter'i yonetici yetkisiyle baslatir; araclar\oyunla-baslat-kur.ps1" -Force | Out-Null
Write-Output ("kuruldu: " + $ad + " (" + $kullanici + ", oturum acilisinda, yonetici) -> " + $Exe)
if ($Baslat) { Start-ScheduledTask -TaskName $ad; Start-Sleep -Seconds 2 }
Write-Output ("durum: " + (Get-ScheduledTask -TaskName $ad).State)
