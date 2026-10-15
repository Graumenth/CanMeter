# CanMeter + ona ait WebView2 süreçlerinin RAM'i ve 10 sn CPU kullanımı.
$kok = Get-Process canmeter-app -ErrorAction SilentlyContinue | Select-Object -First 1
if (-not $kok) { Write-Output "canmeter-app yok"; exit 1 }
$hepsi = Get-CimInstance Win32_Process
$agac = @($kok.Id)
do {
  $yeni = $hepsi | Where-Object { $agac -contains $_.ParentProcessId -and $agac -notcontains $_.ProcessId } | Select-Object -ExpandProperty ProcessId
  $agac += $yeni
} while ($yeni.Count -gt 0)
$sure = 10
$t0 = @{}
foreach ($id in $agac) { $p = Get-Process -Id $id -ErrorAction SilentlyContinue; if ($p) { $t0[$id] = $p.TotalProcessorTime.TotalMilliseconds } }
Start-Sleep -Seconds $sure
$toplamRam = 0; $toplamOzel = 0; $cpuMs = 0
foreach ($id in $agac) {
  $p = Get-Process -Id $id -ErrorAction SilentlyContinue
  if (-not $p) { continue }
  $ram = [math]::Round($p.WorkingSet64 / 1MB, 1); $ozel = [math]::Round($p.PrivateMemorySize64 / 1MB, 1)
  $toplamRam += $ram; $toplamOzel += $ozel
  if ($t0.ContainsKey($id)) { $cpuMs += ($p.TotalProcessorTime.TotalMilliseconds - $t0[$id]) }
  Write-Output ("{0,-22} PID {1,-6} WS {2,7} MB  ozel {3,7} MB" -f $p.ProcessName, $id, $ram, $ozel)
}
$cekirdek = [Environment]::ProcessorCount
$cpuYuzde = [math]::Round($cpuMs / ($sure * 1000) * 100, 2)
Write-Output ("TOPLAM: {0} surec, WS {1} MB, ozel {2} MB; {3} sn CPU: {4}% tek cekirdek esdegeri ({5} cekirdekli makinede %{6})" -f $agac.Count, $toplamRam, $toplamOzel, $sure, $cpuYuzde, $cekirdek, [math]::Round($cpuYuzde / $cekirdek, 3))
