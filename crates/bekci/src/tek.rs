//! Tek bekçi: adlı mutex (oturum başına `Local\`) ile ikinci bekçi hemen çıkar. Görev ve elle açılan exe
//! aynı anda çalışmasın (iki bekçi iki kez başlatma ve kapatma denerdi). Mutex OLUŞTURULAMAZSA (erişim vb.)
//! karar verilemez, bekçi çalışmaya devam eder: kilit yokluğu bekçiyi durdurmaz.

use windows_sys::Win32::Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GetLastError, HANDLE};
use windows_sys::Win32::System::Threading::CreateMutexW;

/// Bekçi kilidinin adı; oturum başına (`Local\`), makine geneli değil.
pub const AD: &str = r"Local\CanMeter.bekci";

/// Alınmış kilit; süreç bitene dek elde tutulur, düşünce (Drop) serbest kalır.
pub struct Kilit {
    tutamac: HANDLE,
}

/// Kilidi almaya çalışır. `None` = başka bekçi var (ERROR_ALREADY_EXISTS). Mutex oluşturulamazsa
/// (NULL tutamaç) `Some` döner, tutamaç null kalır: belirsizlikte çalışmaya devam.
pub fn al(ad: &str) -> Option<Kilit> {
    let ad16: Vec<u16> = ad.encode_utf16().chain(Some(0)).collect();
    // GÜVENLİK: ad NUL sonlu UTF-16, çağrı boyunca yaşıyor; öznitelik null (varsayılan güvenlik).
    let h = unsafe { CreateMutexW(std::ptr::null(), 0, ad16.as_ptr()) };
    if h.is_null() {
        return Some(Kilit { tutamac: h });
    }
    // GÜVENLİK: bu thread'in son hata kodunu okur; hemen önce CreateMutexW çağrıldı, arada API yok.
    if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
        // GÜVENLİK: h az önce CreateMutexW'den döndü, geçerli; tek kez burada kapatılıyor.
        unsafe { CloseHandle(h) };
        return None;
    }
    Some(Kilit { tutamac: h })
}

impl Drop for Kilit {
    fn drop(&mut self) {
        if !self.tutamac.is_null() {
            // GÜVENLİK: tutamaç al() içinde alındı, null değil, yalnız bu yapıya ait; Drop bir kez çalışır.
            unsafe { CloseHandle(self.tutamac) };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn ad(ek: &str) -> String {
        format!(r"Local\CanMeter.bekci.test.{}.{ek}", std::process::id())
    }

    #[test]
    fn ilk_alir() {
        let kilit = al(&ad("ilk")).expect("ilk kilit alınmalı");
        assert!(!kilit.tutamac.is_null());
    }

    #[test]
    fn ikinci_alamaz_birakinca_alir() {
        let ad = ad("ikinci");
        let ilk = al(&ad).unwrap();
        assert!(al(&ad).is_none());
        drop(ilk);
        assert!(al(&ad).is_some());
    }

    #[test]
    fn farkli_adlar_engellemez() {
        let a = al(&ad("a"));
        let b = al(&ad("b"));
        assert!(a.is_some() && b.is_some());
    }
}
