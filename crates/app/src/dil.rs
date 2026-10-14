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
