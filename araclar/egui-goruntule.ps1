# egui overlay'i verilen skin/dil ile açar, pencere dikdörtgenini PNG'ye kaydeder, kapatır.
param([string]$Skin = "gece", [string]$Dil = "tr", [string]$Exe = (Join-Path $PSScriptRoot "..\target\release\canmeter-app.exe"), [string]$Cikti = "", [switch]$Demo)
if (-not $Cikti) { $Cikti = Join-Path $env:TEMP ("egui-" + $Skin + "-" + $Dil + ".png") }
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
Add-Type @"
using System; using System.Runtime.InteropServices;
public struct RECT { public int L, T, R, B; }
public static class W {
  [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern IntPtr FindWindow(string c, string t);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
}
"@
$liste = @("--skin", $Skin, "--dil", $Dil); if ($Demo) { $liste += "--demo" }
$p = Start-Process -FilePath $Exe -ArgumentList $liste -PassThru
Start-Sleep -Seconds 4
$h = [W]::FindWindow([NullString]::Value, "CanMeter")
if ($h -eq [IntPtr]::Zero) { Stop-Process -Id $p.Id -Force; Write-Output "pencere bulunamadi"; exit 1 }
$r = New-Object RECT; [void][W]::GetWindowRect($h, [ref]$r)
$w = $r.R - $r.L; $hh = $r.B - $r.T
$bmp = New-Object System.Drawing.Bitmap $w, $hh
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.CopyFromScreen($r.L, $r.T, 0, 0, (New-Object System.Drawing.Size $w, $hh))
$bmp.Save($Cikti, [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose(); $bmp.Dispose()
$pp = Get-Process -Id $p.Id -ErrorAction SilentlyContinue
$ram = if ($pp) { [math]::Round($pp.PrivateMemorySize64 / 1MB, 1) } else { -1 }
Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
Write-Output ("kaydedildi: " + $Cikti + " " + $w + "x" + $hh + " ozel " + $ram + " MB")
