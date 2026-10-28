//! Süreç adları: Toolhelp32 anlık görüntüsüyle çalışan süreçlerin exe adlarını küçük harfle verir.
//! Oyuna dokunmaz: hiçbir sürece tutamaç açılmaz, yalnız sistem listesi okunur. `unsafe` yalnız bu dosyada.

use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};

/// Çalışan tüm süreçlerin exe adları (küçük harf, örn. "aion2.exe"). Anlık görüntü alınamazsa boş liste.
pub fn adlar() -> Vec<String> {
    // GÜVENLİK: parametreler sabit; dönen tutamaç aşağıda denetlenir ve tek kez kapatılır.
    let h = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if h == INVALID_HANDLE_VALUE || h.is_null() {
        return Vec::new();
    }
    // GÜVENLİK: PROCESSENTRY32W yalnız tamsayı ve dizi alanlı C yapısı; tümü sıfır geçerli değerdir.
    let mut e: PROCESSENTRY32W = unsafe { core::mem::zeroed() };
    e.dwSize = size_of::<PROCESSENTRY32W>() as u32;
    let mut liste = Vec::new();
    // GÜVENLİK: h geçerli anlık görüntü, e doğru dwSize ile yazılabilir yerel yapı.
    let mut var = unsafe { Process32FirstW(h, &mut e) } != 0;
    while var {
        let ad = &e.szExeFile;
        let son = ad.iter().position(|&c| c == 0).unwrap_or(ad.len());
        liste.push(String::from_utf16_lossy(&ad[..son]).to_lowercase());
        // GÜVENLİK: h hâlâ açık, e aynı yerel yapı; çağrı yalnız e içine yazar.
        var = unsafe { Process32NextW(h, &mut e) } != 0;
    }
    // GÜVENLİK: h bu fonksiyonda açıldı ve burada yalnız bir kez kapatılır.
    unsafe { CloseHandle(h) };
    liste
}

/// Oyun açık mı: tam ad eşleşmesi (`oyun` küçük harf, örn. "aion2.exe").
pub fn oyun_var(adlar: &[String], oyun: &str) -> bool {
    adlar.iter().any(|a| a == oyun)
}

/// Meter açık mı: önek eşleşmesi (`onek` küçük harf, örn. "canmeter-app"); yeni derlemede çalışan exe
/// `canmeter-app-calisan-N.exe` adını aldığı için önek şart.
pub fn meter_var(adlar: &[String], onek: &str) -> bool {
    adlar.iter().any(|a| a.starts_with(onek))
}

#[cfg(test)]
mod test {
    use super::*;

    fn liste(x: &[&str]) -> Vec<String> {
        x.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn oyun_tam_eslesme() {
        let l = liste(&["steam.exe", "aion2.exe"]);
        assert!(oyun_var(&l, "aion2.exe"));
        assert!(!oyun_var(&l, "aion2"));
    }

    #[test]
    fn meter_onek() {
        assert!(meter_var(
            &liste(&["canmeter-app-calisan-1.exe"]),
            "canmeter-app"
        ));
        assert!(!meter_var(&liste(&["canmeter-rapor.exe"]), "canmeter-app"));
    }

    #[test]
    fn gercek_liste() {
        let l = adlar();
        assert!(!l.is_empty());
        assert!(l.iter().all(|a| *a == a.to_lowercase()));
        assert!(l.iter().any(|a| a.ends_with(".exe")));
    }
}
