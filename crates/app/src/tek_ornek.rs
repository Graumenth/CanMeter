//! Tek örnek (0.8.8): canlı kipte ikinci meter açılmaz; iki overlay ve çift paket okuma olmasın. Adlı mutex oturum
//! başına (`Local\...`). Kendini yenileme için eski süreç guard'ı yeni exe'yi açmadan önce düşürür, yeni süreç
//! `bekle` süresince 100 ms aralıkla yeniden dener. `--demo` ve `--kayit` kipleri kilit almaz (çağıran karar verir).

use std::time::{Duration, Instant};

use windows_sys::Win32::Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GetLastError, HANDLE};
use windows_sys::Win32::System::Threading::CreateMutexW;

/// Canlı meter kilidinin adı; oturum başına (`Local\`), makine geneli değil.
pub const CANLI_AD: &str = r"Local\CanMeter.canli";

/// Yeniden deneme aralığı.
const ARALIK: Duration = Duration::from_millis(100);

/// Alınmış kilit; düşünce (Drop) serbest kalır.
pub struct TekOrnek {
    tutamac: HANDLE,
}

// GÜVENLİK: tutamaç süreç geneli geçerli ve yalnız bu yapıya ait; hangi thread kapatırsa kapatsın doğrudur.
unsafe impl Send for TekOrnek {}

/// Kilidi almaya çalışır: hemen alınamazsa `bekle` boyunca 100 ms'de bir yeniden dener. `None` = başka örnek var
/// (ya da mutex hiç oluşturulamadı; o durumda da None döner ve stderr'e yazar).
pub fn al(ad: &str, bekle: Duration) -> Option<TekOrnek> {
    let ad16: Vec<u16> = ad.encode_utf16().chain(Some(0)).collect();
    let bas = Instant::now();
    loop {
        match dene(&ad16) {
            Ok(kilit) => return Some(kilit),
            Err(false) => {
                eprintln!("tek örnek: mutex oluşturulamadı ({ad})");
                return None;
            }
            Err(true) if bas.elapsed() >= bekle => return None,
            Err(true) => std::thread::sleep(ARALIK),
        }
    }
}

/// Tek deneme. `Err(true)` = mutex zaten var (başka örnek), `Err(false)` = oluşturulamadı.
fn dene(ad16: &[u16]) -> Result<TekOrnek, bool> {
    debug_assert_eq!(ad16.last(), Some(&0));
    // GÜVENLİK: ad NUL sonlu UTF-16, çağrı boyunca yaşıyor; öznitelik null (varsayılan güvenlik).
    let h = unsafe { CreateMutexW(std::ptr::null(), 0, ad16.as_ptr()) };
    if h.is_null() {
        return Err(false);
    }
    // GÜVENLİK: bu thread'in son hata kodunu okur; hemen önce CreateMutexW çağrıldı, arada API yok.
    if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
        // GÜVENLİK: h az önce CreateMutexW'den döndü, geçerli; tek kez burada kapatılıyor.
        unsafe { CloseHandle(h) };
        return Err(true);
    }
    Ok(TekOrnek { tutamac: h })
}

impl Drop for TekOrnek {
    fn drop(&mut self) {
        // GÜVENLİK: tutamaç dene() içinde alındı, yalnız bu yapıya ait; Drop bir kez çalışır.
        unsafe { CloseHandle(self.tutamac) };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn ad(ek: &str) -> String {
        format!(r"Local\CanMeter.test.{}.{ek}", std::process::id())
    }

    #[test]
    fn ilk_alir() {
        let ad = ad("ilk");
        assert!(al(&ad, Duration::ZERO).is_some());
    }

    #[test]
    fn ikinci_alamaz_birakinca_alir() {
        let ad = ad("ikinci");
        let ilk = al(&ad, Duration::ZERO).unwrap();
        assert!(al(&ad, Duration::ZERO).is_none());
        drop(ilk);
        assert!(al(&ad, Duration::ZERO).is_some());
    }

    #[test]
    fn bekleyerek_alir() {
        let ad = ad("bekle");
        let ilk = al(&ad, Duration::ZERO).unwrap();
        let t = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(300));
            drop(ilk);
        });
        assert!(al(&ad, Duration::from_secs(2)).is_some());
        t.join().unwrap();
    }
}
