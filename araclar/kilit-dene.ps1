# Kilit düğmesi denemesi (oyun kapalıyken; Gece skin'i, ölçek 1, düğme sağ üstte): kilitle, uzaklaş, düğmeye gel, tıkla, aç. WS_EX_TRANSPARENT izlenir.
param([string]$Exe = (Join-Path $PSScriptRoot "..\target\release\canmeter-app.exe"))
Add-Type @"
using System; using System.Runtime.InteropServices;
public struct RECT { public int L, T, R, B; }
public static class W {
  public delegate bool EnumProc(IntPtr h, IntPtr l);
  [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc p, IntPtr l);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern IntPtr GetWindowLongPtrW(IntPtr h, int i);
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint f, int dx, int dy, uint d, UIntPtr e);
  public static IntPtr Bul(uint pid) { IntPtr s = IntPtr.Zero; EnumWindows((h, l) => { uint p; GetWindowThreadProcessId(h, out p); if (p == pid && IsWindowVisible(h)) { s = h; return false; } return true; }, IntPtr.Zero); return s; }
}
"@
function Gecirgen($h) { ([long][W]::GetWindowLongPtrW($h, -20) -band 0x20) -ne 0 }
function Tikla { [W]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 60; [W]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero) }
$p = Start-Process -FilePath $Exe -ArgumentList @("--demo", "--skin", "gece") -PassThru
Start-Sleep -Seconds 4
$h = [W]::Bul([uint32]$p.Id)
if ($h -eq [IntPtr]::Zero) { Stop-Process -Id $p.Id -Force; Write-Output "pencere yok"; exit 1 }
$r = New-Object RECT; [void][W]::GetWindowRect($h, [ref]$r)
$kx = $r.R - 14; $ky = $r.T + 27
Write-Output ("1 baslangic gecirgen: " + (Gecirgen $h))
[void][W]::SetCursorPos($kx, $ky); Start-Sleep -Milliseconds 200; Tikla; Start-Sleep -Milliseconds 500
Write-Output ("2 kilit tiklandi, gecirgen (true beklenir): " + (Gecirgen $h))
[void][W]::SetCursorPos($r.L + 100, $r.T + 200); Start-Sleep -Milliseconds 400
Write-Output ("3 imlec satirlarda, gecirgen (true beklenir): " + (Gecirgen $h))
[void][W]::SetCursorPos($kx, $ky); Start-Sleep -Milliseconds 400
Write-Output ("4 imlec kilit dugmesinde, gecirgen (false beklenir): " + (Gecirgen $h))
Tikla; Start-Sleep -Milliseconds 500
Write-Output ("5 dugme tiklandi, gecirgen (false beklenir): " + (Gecirgen $h))
[void][W]::SetCursorPos($r.L + 100, $r.T + 200); Start-Sleep -Milliseconds 400
Write-Output ("6 kilit acik, imlec satirlarda, gecirgen (false beklenir): " + (Gecirgen $h))
$pp = Get-Process -Id $p.Id -ErrorAction SilentlyContinue
if ($pp) { Write-Output ("ozel RAM MB: " + [math]::Round($pp.PrivateMemorySize64 / 1MB, 1)) }
Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
