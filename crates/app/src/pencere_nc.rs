//! Çerçevesiz pencerede winit, WM_NCCALCSIZE "gölge hilesi" ile üstte 1 px istemci dışı şerit bırakır; DWM onu açık
//! renk çizer (overlay'in üstündeki ince çizgi). Pencere yordamına girip bu iletide istemci alanını pencerenin tamamı
//! yapıyoruz; diğer iletiler winit'in yordamına aynen gider.

use std::sync::atomic::{AtomicIsize, Ordering};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, GWLP_WNDPROC, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER,
    SetWindowLongPtrW, SetWindowPos, WM_NCCALCSIZE, WNDPROC,
};

static ESKI_YORDAM: AtomicIsize = AtomicIsize::new(0);

unsafe extern "system" fn yordam(hwnd: HWND, ileti: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    if ileti == WM_NCCALCSIZE && w != 0 {
        // İstemci alanı = pencere dikdörtgeni: üstte şerit yok.
        return 0;
    }
    // Saklanan değer SetWindowLongPtrW'nin döndürdüğü geçerli pencere yordamıdır (sıfır olamaz: 0 iken çağrılmaz).
    let eski: WNDPROC = unsafe { std::mem::transmute(ESKI_YORDAM.load(Ordering::Relaxed)) };
    unsafe { CallWindowProcW(eski, hwnd, ileti, w, l) }
}

/// Oluşturma bağlamında bir kez; aynı iş parçacığında çağrıldığı için yordam değişimi ile saklama arasına ileti girmez.
pub fn uygula(hwnd: HWND) {
    unsafe {
        let eski = SetWindowLongPtrW(hwnd, GWLP_WNDPROC, yordam as *const () as isize);
        if eski == 0 {
            return;
        }
        ESKI_YORDAM.store(eski, Ordering::Relaxed);
        // Çerçeve yeniden hesaplansın (WM_NCCALCSIZE şimdi bizden geçer).
        SetWindowPos(
            hwnd,
            std::ptr::null_mut(),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
        );
    }
}
