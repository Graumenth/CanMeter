//! Skin sözleşmesi (Strategy): her görünüm aynı veriyi alır, aynı eylemleri döndürür.
//! Skin yalnız çizer; pencere komutu, ayar kaydı, kısayol kabukta (`uygulama.rs`) kalır.
//! Bu dosya donmuştur: skin işleri ve kabuk işi buna göre yazılır, değiştirilmez.

use canmeter_core::stats::SavasAnlik;
use egui::Color32;

use crate::dil::Dil;

pub mod kayit;

pub mod gece;
pub mod kehribar;
pub mod komur;
pub mod kor;
pub mod kusak;
pub mod obsidyen;
pub mod sis;

/// Ayarlardan gelen panel zemini. `renk` yoksa skin'in kendi rengi kullanılır.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Zemin {
    pub renk: Option<Color32>,
    /// 0.3..=1.0
    pub opaklik: f32,
}

impl Zemin {
    /// Skin'in panel dolgusu: seçili renk (ya da `varsayilan`) + ayarlanan opaklık.
    pub fn dolgu(&self, varsayilan: Color32) -> Color32 {
        let r = self.renk.unwrap_or(varsayilan);
        let a = (self.opaklik.clamp(0.0, 1.0) * 255.0).round() as u8;
        Color32::from_rgba_unmultiplied(r.r(), r.g(), r.b(), a)
    }
}

/// Bir karede skin'e verilen her şey.
pub struct Baglam<'a> {
    /// `None`: savaş yok, bekleme görünümü çizilir.
    pub savas: Option<&'a SavasAnlik>,
    pub dil: Dil,
    pub kilitli: bool,
    /// Üst özet satırı (solda toplam DPS, sağda toplam hasar; yalnız yazı).
    pub toplam_goster: bool,
    /// Hedef satırı (ad, HP çubuğu, kalan süre). Sahip isteğiyle eklendi (0.7.0); sözleşmenin tek genişlemesi.
    pub hedef_goster: bool,
    pub zemin: Zemin,
    /// Veri demo kaynağından geliyor (alt bilgide gösterilir).
    pub demo: bool,
    pub surum: &'static str,
}

/// Skin'in kullanıcıdan aldığı istekler; kabuk uygular.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Eylemler {
    pub kilit_degistir: bool,
    pub ayarlar_ac: bool,
    /// Başlık alanından sürükleme başladı; kabuk pencereyi taşır.
    pub surukle: bool,
}

impl Eylemler {
    pub fn birlestir(self, o: Eylemler) -> Eylemler {
        Eylemler {
            kilit_degistir: self.kilit_degistir || o.kilit_degistir,
            ayarlar_ac: self.ayarlar_ac || o.ayarlar_ac,
            surukle: self.surukle || o.surukle,
        }
    }
}

pub trait Skin: Sync {
    /// Kayıt ve ayar anahtarı ("gece").
    fn ad(&self) -> &'static str;
    /// Kullanıcıya görünen ad ("Gece").
    fn etiket(&self) -> &'static str;
    /// Ayarlarda renk seçilmemişse panel zemini.
    fn zemin_rengi(&self) -> Color32;
    /// Tam görünümü `ui` içine çizer. Animasyon yok, kare başına ayırma en az.
    fn ciz(&self, ui: &mut egui::Ui, b: &Baglam) -> Eylemler;
}
