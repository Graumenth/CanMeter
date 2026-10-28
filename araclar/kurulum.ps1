# Kurulu kopya kökü (CC kuralı `CanMeterCC\kural-kurulu-kopya.md`): ürün target\release'ten değil buradan çalışır.
# Betikler bunu dot-source eder: . (Join-Path $PSScriptRoot "kurulum.ps1"). Yol başka yere elle yazılmaz.
$KurulumKok = "D:\CanMeter"
$MeterKlasor = Join-Path $KurulumKok "Meter"
$HubKlasor = Join-Path $KurulumKok "CanMeter Hub"
