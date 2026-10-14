//! Arayüz dili. Sözlükler her skin'in `metin.rs`'inde ve ayarlar panelinde `const` olarak durur.

use std::fmt::Display;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Dil {
    #[default]
    Tr,
    En,
}

impl Dil {
    pub const HEPSI: [Dil; 2] = [Dil::Tr, Dil::En];

    /// Ayar dosyası ve `--dil` değeri.
    pub fn kod(self) -> &'static str {
        match self {
            Dil::Tr => "tr",
            Dil::En => "en",
        }
    }

    pub fn ayristir(kod: &str) -> Option<Dil> {
        Self::HEPSI.into_iter().find(|d| d.kod() == kod)
    }

    /// Ctrl+Shift+L döngüsü.
    pub fn sonraki(self) -> Dil {
        match self {
            Dil::Tr => Dil::En,
            Dil::En => Dil::Tr,
        }
    }

    /// İlk açılış varsayılanı: bilgisayarın arayüz dili Türkçe ise Tr, başka her dilde En (sahip kararı).
    pub fn sistem() -> Dil {
        const LANG_TURKISH: u16 = 0x1f;
        match sistem_langid() {
            Some(id) => {
                if id & 0x3ff == LANG_TURKISH {
                    Dil::Tr
                } else {
                    Dil::En
                }
            }
            None => {
                if std::env::var("LANG").is_ok_and(|l| l.starts_with("tr")) {
                    Dil::Tr
                } else {
                    Dil::En
                }
            }
        }
    }
}

/// Windows kullanıcı arayüzü dili (LANGID); başka platformda yok.
#[cfg(windows)]
fn sistem_langid() -> Option<u16> {
    // SAFETY: parametresiz Win32 çağrısı, yalnız bir u16 döndürür.
    Some(unsafe { windows_sys::Win32::Globalization::GetUserDefaultUILanguage() })
}

#[cfg(not(windows))]
fn sistem_langid() -> Option<u16> {
    None
}

/// `{0}`, `{1}` yer tutucularını sırayla doldurur (sözlük şablonları için).
pub fn bicimle(sablon: &str, degerler: &[&dyn Display]) -> String {
    let mut cikti = String::with_capacity(sablon.len() + 16);
    let mut kalan = sablon;
    while let Some(bas) = kalan.find('{') {
        cikti.push_str(&kalan[..bas]);
        let Some(son) = kalan[bas..].find('}') else {
            cikti.push_str(&kalan[bas..]);
            return cikti;
        };
        match kalan[bas + 1..bas + son]
            .parse::<usize>()
            .ok()
            .and_then(|i| degerler.get(i))
        {
            Some(d) => cikti.push_str(&d.to_string()),
            None => cikti.push_str(&kalan[bas..=bas + son]),
        }
        kalan = &kalan[bas + son + 1..];
    }
    cikti.push_str(kalan);
    cikti
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sistem_dili_gecerli() {
        assert!(Dil::HEPSI.contains(&Dil::sistem()));
    }

    #[test]
    fn kod_gidis_donus() {
        for d in Dil::HEPSI {
            assert_eq!(Dil::ayristir(d.kod()), Some(d));
        }
        assert_eq!(Dil::ayristir("de"), None);
    }

    #[test]
    fn bicimle_yer_tutucu() {
        assert_eq!(bicimle("{0} oyuncu, {1}", &[&6, &"demo"]), "6 oyuncu, demo");
        assert_eq!(bicimle("{9} yok", &[&1]), "{9} yok");
    }
}
