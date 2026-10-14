//! Yerel overlay penceresi ve Windows DWM kenarlık ayarı.

pub fn secenekler() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("CanMeter")
            // Genişlik kullanıcıdan (kenardan sürükleme = ölçek), yükseklik içerikten; bkz. boyut.rs.
            .with_inner_size([440.0, 440.0])
            .with_min_inner_size([220.0, 40.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(true),
        // 4 Hz çizimde vsync beklemesi yalnız CPU yakar; kare aralığını kabuk belirler.
        glow_options: eframe::egui_glow::GlowConfiguration {
            vsync: false,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[cfg(windows)]
pub fn kenarlik_kaldir(cc: &eframe::CreationContext<'_>) {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows_sys::Win32::Graphics::Dwm::DwmSetWindowAttribute;

    let Ok(pencere) = cc.window_handle() else {
        return;
    };
    let RawWindowHandle::Win32(pencere) = pencere.as_raw() else {
        return;
    };
    const DWMWA_BORDER_COLOR: u32 = 34;
    const DWMWA_COLOR_NONE: u32 = 0xFFFFFFFE;
    // HWND oluşturma bağlamına aittir; renk işaretçisi çağrı boyunca geçerlidir.
    unsafe {
        let _ = DwmSetWindowAttribute(
            pencere.hwnd.get() as _,
            DWMWA_BORDER_COLOR,
            (&DWMWA_COLOR_NONE as *const u32).cast(),
            4,
        );
    }
}

#[cfg(not(windows))]
pub fn kenarlik_kaldir(_: &eframe::CreationContext<'_>) {}
