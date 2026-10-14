//! Webview tarafından çağrılan küçük Tauri komutları.
//! Pencere komutu overlay davranışını, sürüm komutu çekirdek bilgisini verir.
//! Komutlar doğrudan çekirdeğin hesaplama durumunu değiştirmez.
//! Kilit açıkken pencere fare olaylarını alır.
//! Kilitliyken tıklamalar oyuna geçer.

/// Overlay penceresinin fare olaylarını oyuna geçirip geçirmeyeceğini ayarlar.
#[tauri::command]
pub fn overlay_kilit(pencere: tauri::Window, kilit: bool) -> Result<(), String> {
    pencere
        .set_ignore_cursor_events(kilit)
        .map_err(|hata| hata.to_string())
}

// Sürüm tek kaynaktan, çekirdek paketinden alınır.
/// Çekirdeğin sürümünü döndürür.
#[tauri::command]
pub fn surum() -> &'static str {
    canmeter_core::surum()
}
