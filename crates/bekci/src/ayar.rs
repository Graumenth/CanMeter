//! Komut satırı ayarı: bağımlılıksız elle ayrıştırma. Varsayılanlar ürün düzenine göre: oyun `aion2.exe`,
//! meter bekçinin kendi klasöründeki `canmeter-app.exe` (geliştirmede `target\release`, kurulumda `Meter\`).

use std::path::PathBuf;

pub struct Ayar {
    /// Oyun süreç adı, küçük harf (`--oyun`).
    pub oyun: String,
    /// Meter süreç adı öneki (`--meter`); yeni derlemede `canmeter-app-calisan-N.exe` olur.
    pub meter_onek: String,
    /// Başlatılacak meter exe'si (`--exe`).
    pub exe: PathBuf,
    /// Bakış aralığı, saniye (`--aralik`, 0 ya da geçersizse 5).
    pub aralik_sn: u64,
    /// Başlatmaz, yalnız iz yazar (`--kuru`).
    pub kuru: bool,
    /// Karar izi dosyası (`--iz`).
    pub iz: Option<PathBuf>,
}

/// Sıradaki argümanı alır; yoksa boş metin döner.
fn sonraki(it: &mut std::slice::Iter<'_, String>) -> String {
    it.next().cloned().unwrap_or_default()
}

/// Argüman dilimini ayara çevirir; bilinmeyen seçenekler yok sayılır.
pub fn ayristir(args: &[String]) -> Ayar {
    let varsayilan_exe = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("canmeter-app.exe")))
        .unwrap_or_else(|| PathBuf::from("canmeter-app.exe"));
    let mut ayar = Ayar {
        oyun: "aion2.exe".to_string(),
        meter_onek: "canmeter-app".to_string(),
        exe: varsayilan_exe,
        aralik_sn: 5,
        kuru: false,
        iz: None,
    };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--oyun" => ayar.oyun = sonraki(&mut it).to_lowercase(),
            "--meter" => ayar.meter_onek = sonraki(&mut it),
            "--exe" => ayar.exe = PathBuf::from(sonraki(&mut it)),
            "--aralik" => {
                ayar.aralik_sn = sonraki(&mut it)
                    .parse::<u64>()
                    .ok()
                    .filter(|&n| n > 0)
                    .unwrap_or(5)
            }
            "--kuru" => ayar.kuru = true,
            "--iz" => ayar.iz = Some(PathBuf::from(sonraki(&mut it))),
            _ => {}
        }
    }
    ayar
}

/// Sürecin kendi argümanlarından ayar.
pub fn oku() -> Ayar {
    ayristir(&std::env::args().skip(1).collect::<Vec<String>>())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn varsayilanlar() {
        let a = ayristir(&[]);
        assert_eq!(
            (a.oyun.as_str(), a.aralik_sn, a.kuru),
            ("aion2.exe", 5, false)
        );
    }
    #[test]
    fn oyun_kucuk_harfe_iner() {
        assert_eq!(
            ayristir(&["--oyun".into(), "AION2.exe".into()]).oyun,
            "aion2.exe"
        );
    }
    #[test]
    fn aralik_sifir_ise_bes() {
        assert_eq!(ayristir(&["--aralik".into(), "0".into()]).aralik_sn, 5);
    }
    #[test]
    fn kuru_ve_iz_doldurulur() {
        let a = ayristir(&["--kuru".into(), "--iz".into(), "C:\\x.txt".into()]);
        assert_eq!((a.kuru, a.iz), (true, Some(PathBuf::from("C:\\x.txt"))));
    }
}
