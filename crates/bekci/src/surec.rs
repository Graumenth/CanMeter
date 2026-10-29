//! Süreçler: Toolhelp32 anlık görüntüsüyle çalışan süreçlerin exe adlarını (küçük harf) ve kimliklerini verir.
//! Oyuna dokunmaz: hiçbir sürece tutamaç açılmaz, yalnız sistem listesi okunur. Kimlikler yalnız meter'ın
//! penceresini bulmak için (`kapat.rs`) kullanılır.

use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};

/// Çalışan bir süreç: küçük harf exe adı ve kimliği.
pub struct Surec {
    pub ad: String,
    pub pid: u32,
}

/// Çalışan tüm süreçler (ad küçük harf, örn. "aion2.exe"). Anlık görüntü alınamazsa boş liste.
pub fn surecler() -> Vec<Surec> {
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
        liste.push(Surec {
            ad: String::from_utf16_lossy(&ad[..son]).to_lowercase(),
            pid: e.th32ProcessID,
        });
        // GÜVENLİK: h hâlâ açık, e aynı yerel yapı; çağrı yalnız e içine yazar.
        var = unsafe { Process32NextW(h, &mut e) } != 0;
    }
    // GÜVENLİK: h bu fonksiyonda açıldı ve burada yalnız bir kez kapatılır.
    unsafe { CloseHandle(h) };
    liste
}

/// Oyun açık mı: tam ad eşleşmesi (`oyun` küçük harf, örn. "aion2.exe").
pub fn oyun_var(s: &[Surec], oyun: &str) -> bool {
    s.iter().any(|x| x.ad == oyun)
}

/// Meter açık mı: önek eşleşmesi (`onek` küçük harf, örn. "canmeter-app"); yeni derlemede çalışan exe
/// `canmeter-app-calisan-N.exe` adını aldığı için önek şart.
pub fn meter_var(s: &[Surec], onek: &str) -> bool {
    s.iter().any(|x| x.ad.starts_with(onek))
}

/// Öneki tutan süreçlerin kimlikleri (kapatma için).
pub fn meter_pidler(s: &[Surec], onek: &str) -> Vec<u32> {
    s.iter()
        .filter(|x| x.ad.starts_with(onek))
        .map(|x| x.pid)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn liste(x: &[&str]) -> Vec<Surec> {
        x.iter()
            .enumerate()
            .map(|(i, s)| Surec {
                ad: s.to_string(),
                pid: i as u32 + 1,
            })
            .collect()
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
    fn meter_pidleri_sirayla() {
        let l = liste(&[
            "canmeter-app.exe",
            "aion2.exe",
            "canmeter-rapor.exe",
            "canmeter-app-calisan-2.exe",
        ]);
        assert_eq!(meter_pidler(&l, "canmeter-app"), vec![1, 4]);
        assert!(meter_pidler(&l, "yok").is_empty());
    }

    #[test]
    fn gercek_liste() {
        let l = surecler();
        assert!(!l.is_empty());
        assert!(l.iter().all(|x| x.ad == x.ad.to_lowercase()));
        assert!(l.iter().any(|x| x.ad.ends_with(".exe")));
        assert!(l.iter().any(|x| x.pid != 0));
    }
}
