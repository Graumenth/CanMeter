//! React ayarlar sözlüğünün Türkçe ve İngilizce karşılıkları.
use crate::dil::Dil;

pub struct Sozluk {
    pub ayarlar: &'static str,
    pub kapat: &'static str,
    pub dil: &'static str,
    pub gorunum: &'static str,
    pub seffaflik: &'static str,
    pub zemin_rengi: &'static str,
    pub varsayilan: &'static str,
    pub toplam_gostergesi: &'static str,
    pub kisayollar: &'static str,
    pub gorunum_kisayol: &'static str,
    pub dil_kisayol: &'static str,
}

pub const TR: Sozluk = Sozluk {
    ayarlar: "Ayarlar",
    kapat: "Kapat",
    dil: "Dil",
    gorunum: "Görünüm",
    seffaflik: "Şeffaflık",
    zemin_rengi: "Zemin rengi",
    varsayilan: "Varsayılan",
    toplam_gostergesi: "Toplam göstergesi",
    kisayollar: "Kısayollar",
    gorunum_kisayol: "Ctrl+Shift+S görünüm",
    dil_kisayol: "Ctrl+Shift+L dil",
};

pub const EN: Sozluk = Sozluk {
    ayarlar: "Settings",
    kapat: "Close",
    dil: "Language",
    gorunum: "Appearance",
    seffaflik: "Opacity",
    zemin_rengi: "Background color",
    varsayilan: "Default",
    toplam_gostergesi: "Totals display",
    kisayollar: "Shortcuts",
    gorunum_kisayol: "Ctrl+Shift+S appearance",
    dil_kisayol: "Ctrl+Shift+L language",
};

pub fn metin(dil: Dil) -> &'static Sozluk {
    match dil {
        Dil::Tr => &TR,
        Dil::En => &EN,
    }
}
