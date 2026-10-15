# Masaüstünde fareyle sağ kenar tutamacından boyutlandırma denemesi (oyun kapalıyken). Kendi örneğini açar, ölçer, kapatır.
param([string]$Exe = (Join-Path $PSScriptRoot "..\target\release\canmeter-app.exe"))
Add-Type @"
using System; using System.Runtime.InteropServices; using System.Collections.Generic;
public struct RECT { public int L, T, R, B; }
public static class W {
  public delegate bool EnumProc(IntPtr h, IntPtr l);
  [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc p, IntPtr l);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern IntPtr SendMessage(IntPtr h, uint m, IntPtr w, IntPtr l);
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint f, int dx, int dy, uint d, UIntPtr e);
  public static IntPtr Bul(uint pid) { IntPtr s = IntPtr.Zero; EnumWindows((h, l) => { uint p; GetWindowThreadProcessId(h, out p); if (p == pid && IsWindowVisible(h)) { s = h; return false; } return true; }, IntPtr.Zero); return s; }
}
"@
$p = Start-Process -FilePath $Exe -ArgumentList @("--demo", "--skin", "gece") -PassThru
Start-Sleep -Seconds 4
$h = [W]::Bul([uint32]$p.Id)
if ($h -eq [IntPtr]::Zero) { Stop-Process -Id $p.Id -Force; Write-Output "pencere yok"; exit 1 }
$r = New-Object RECT; [void][W]::GetWindowRect($h, [ref]$r)
Write-Output ("baslangic: {0},{1} {2}x{3}" -f $r.L, $r.T, ($r.R - $r.L), ($r.B - $r.T))
$my = [int](($r.T + $r.B) / 2)
foreach ($dx in 1, 3, 6, 9, 20) {
  $x = $r.R - $dx
  $lp = [IntPtr](($my -shl 16) -bor ($x -band 0xFFFF))
  $ht = [W]::SendMessage($h, 0x84, [IntPtr]::Zero, $lp)
  Write-Output ("NCHITTEST sag kenar -{0}px: {1}" -f $dx, $ht)
}
$lp = [IntPtr]((($r.B - 3) -shl 16) -bor (($r.R - 3) -band 0xFFFF))
Write-Output ("NCHITTEST sag alt kose -3px: {0}" -f [W]::SendMessage($h, 0x84, [IntPtr]::Zero, $lp))
# Sürükleme: sağ kenardan +120 px
$x0 = $r.R - 3
[void][W]::SetCursorPos($x0, $my); Start-Sleep -Milliseconds 300
[W]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 100
for ($i = 1; $i -le 12; $i++) { [void][W]::SetCursorPos($x0 + $i * 10, $my); Start-Sleep -Milliseconds 40 }
[W]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero)
Start-Sleep -Milliseconds 1500
$r2 = New-Object RECT; [void][W]::GetWindowRect($h, [ref]$r2)
Write-Output ("surukleme sonrasi: {0},{1} {2}x{3}" -f $r2.L, $r2.T, ($r2.R - $r2.L), ($r2.B - $r2.T))
Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
