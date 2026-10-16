//! İki tonlu mürekkep (0.6.2): dolu çubuğun (sınıf rengi + yeşil iyileştirme) üstüne düşen yazı koyu, rayın
//! üstüne düşen yazı açık çizilir. Aynı şekil iki kez, birbirini tamamlayan kırpma alanlarıyla boyanır; her
//! piksel tek tondur, yazı çubuğun ucunda bölünse bile iki yarı da okunur. Gölge yok (sahip 0.4.9'da gölgeyi
//! "hayalet" bulmuştu). Sebep: sarı (Cleric) ve açık yeşil çubukta beyaz sayılar oyunda hiç okunmuyordu.

use egui::{Color32, Painter, Rect, pos2};

/// Çubuğun bittiği x; solu çubuk, sağı ray sayılır.
#[derive(Clone, Copy)]
pub struct Sinir(pub f32);

impl Sinir {
    /// `ciz` iki kez çağrılır: önce çubuk alanına kırpılmış boyacı ve `koyu = true`, sonra ray alanı ve `false`.
    /// Boş kalan taraf atlanır.
    pub fn ikili(self, p: &Painter, ciz: impl Fn(&Painter, bool)) {
        let tum = p.clip_rect();
        if self.0 > tum.min.x {
            let cubuk = Rect::from_min_max(tum.min, pos2(self.0, tum.max.y));
            ciz(&p.with_clip_rect(cubuk), true);
        }
        if self.0 < tum.max.x {
            let ray = Rect::from_min_max(pos2(self.0, tum.min.y), tum.max);
            ciz(&p.with_clip_rect(ray), false);
        }
    }
}

/// Çubukta `koyu_ton`, rayda `acik`.
pub fn ton(koyu: bool, acik: Color32, koyu_ton: Color32) -> Color32 {
    if koyu { koyu_ton } else { acik }
}
