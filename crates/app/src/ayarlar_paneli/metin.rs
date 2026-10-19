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
    pub hedef_cubugu: &'static str,
    pub gecmis_kaydet: &'static str,
    pub kisayollar: &'static str,
    pub gorunum_kisayol: &'static str,
    pub dil_kisayol: &'static str,
    pub kilit_kisayol: &'static str,
    pub yenile_kisayol: &'static str,
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
    hedef_cubugu: "Hedef çubuğu",
    gecmis_kaydet: "Geçmişi kaydet",
    kisayollar: "Kısayollar",
    gorunum_kisayol: "Ctrl+Shift+S görünüm",
    dil_kisayol: "Ctrl+Shift+L dil",
    kilit_kisayol: "Ctrl+Shift+K kilit (oyundayken de)",
    yenile_kisayol: "Ctrl+Shift+R yeniden başlat (yeni derleme)",
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
    hedef_cubugu: "Target bar",
    gecmis_kaydet: "Save history",
    kisayollar: "Shortcuts",
    gorunum_kisayol: "Ctrl+Shift+S appearance",
    dil_kisayol: "Ctrl+Shift+L language",
    kilit_kisayol: "Ctrl+Shift+K lock (works in game)",
    yenile_kisayol: "Ctrl+Shift+R restart (new build)",
};

pub fn metin(dil: Dil) -> &'static Sozluk {
    match dil {
        Dil::Tr => &TR,
        Dil::En => &EN,
    }
}
