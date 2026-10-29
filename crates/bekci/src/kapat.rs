//! Meter'ı nazikçe kapatma: verilen süreçlerin görünür ve başlıklı üst düzey pencerelerine WM_CLOSE gönderilir.
//! NEDEN görünür ve başlıklı: meter eframe/winit; winit her iş parçacığı için üst düzey, başlıksız bir
//! "Winit Thread Event Target" penceresi açar (IsWindowVisible onu da görünür sayar, 2026-10-29 ölçüldü) ve ona
//! gelen WM_CLOSE'u DefWindowProc'a bırakır (pencere yok edilir, olay döngüsü bozulabilir). Başlıklı olan yalnız
//! meter'ın "CanMeter" overlay penceresidir; WM_CLOSE orada winit CloseRequested'a, eframe `on_exit`'e gider
//! (meter `Motor::kapat()` ile savaşı dosyaya yazar).
//! Nazik kapanış budur: TerminateProcess/OpenProcess yok, hiçbir sürece tutamaç açılmaz.
//! `unsafe` yalnız bu dosyada ve `surec.rs`'te.

use windows_sys::Win32::Foundation::{HWND, LPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextLengthW, GetWindowThreadProcessId, IsWindowVisible, PostMessageW,
    WM_CLOSE,
};
use windows_sys::core::BOOL;

/// EnumWindows boyunca geri çağrıya LPARAM ile taşınan bağlam.
struct Baglam<'a> {
    pidler: &'a [u32],
    gonderilen: usize,
}

/// Her üst düzey pencere için çağrılır; hedef süreçlerin görünür ve başlıklı penceresine WM_CLOSE bırakır.
unsafe extern "system" fn pencere(hwnd: HWND, lp: LPARAM) -> BOOL {
    // GÜVENLİK: lp, pencereleri_kapat'taki yerel Baglam'a işaretçi; geri çağrı yalnız o EnumWindows
    // çağrısından gelir, yapı çağrı bitene dek yaşar ve başka hiçbir yerden erişilmez.
    let b = unsafe { &mut *(lp as *mut Baglam) };
    // GÜVENLİK: hwnd EnumWindows'un verdiği pencere; iki çağrı da yalnız okur.
    if unsafe { IsWindowVisible(hwnd) } == 0 || unsafe { GetWindowTextLengthW(hwnd) } == 0 {
        return 1;
    }
    let mut pid = 0u32;
    // GÜVENLİK: hwnd EnumWindows'tan; pid yazılabilir yerel değişken. Tutamaç açılmaz.
    unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
    // GÜVENLİK: PostMessageW kuyruğa bırakır, beklemez; parametreler sabit, işaretçi taşımaz.
    if b.pidler.contains(&pid) && unsafe { PostMessageW(hwnd, WM_CLOSE, 0, 0) } != 0 {
        b.gonderilen += 1;
    }
    1
}

/// Verilen süreçlerin görünür ve başlıklı üst düzey pencerelerine WM_CLOSE gönderir; kaç pencereye gönderildiğini
/// döner.
pub fn pencereleri_kapat(pidler: &[u32]) -> usize {
    if pidler.is_empty() {
        return 0;
    }
    let mut b = Baglam {
        pidler,
        gonderilen: 0,
    };
    // GÜVENLİK: geri çağrı imzası WNDENUMPROC'a uyar; LPARAM b'ye işaretçi ve b bu çağrı süresince
    // yaşar. EnumWindows dönünce işaretçi bir daha kullanılmaz.
    unsafe { EnumWindows(Some(pencere), &mut b as *mut Baglam as LPARAM) };
    b.gonderilen
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bos_liste_sifir() {
        assert_eq!(pencereleri_kapat(&[]), 0);
    }

    #[test]
    fn olmayan_surec_sifir() {
        assert_eq!(pencereleri_kapat(&[u32::MAX]), 0);
    }
}
