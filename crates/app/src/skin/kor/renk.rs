//! Kor mockup'inin renk paleti.

use egui::Color32;

// Panel ve bolum zeminleri.
pub const ZEMIN: Color32 = Color32::from_rgb(0x0c, 0x10, 0x1b);
// Sabitlerde yalnız premultiplied const fn var: rgb = rgb * a / 255 elle çarpıldı.
pub const BASLIK: Color32 = Color32::from_rgba_premultiplied(19, 25, 40, 245);
pub const SATIR_CIFT: Color32 = Color32::from_rgba_premultiplied(6, 6, 6, 6);

// Ince ayiricilar ve ana metin basamaklari.
pub const CIZGI: Color32 = Color32::from_rgba_premultiplied(23, 27, 35, 41);
pub const METIN: Color32 = Color32::from_rgb(0xf2, 0xf5, 0xfc);
pub const METIN_2: Color32 = Color32::from_rgb(0xa9, 0xb4, 0xcc);
pub const METIN_3: Color32 = Color32::from_rgb(0x7b, 0x87, 0xa1);

// Durum ve vurgu renkleri.
pub const VURGU: Color32 = Color32::from_rgb(0x6f, 0x9b, 0xff);
pub const IYILESTIRME: Color32 = Color32::from_rgb(0x5f, 0xe3, 0xa0);
pub const TEHLIKE: Color32 = Color32::from_rgb(0xff, 0x6b, 0x6b);
pub const CANLI: Color32 = Color32::from_rgb(0x3d, 0xdc, 0x84);
pub const OLU: Color32 = Color32::from_rgb(0x8a, 0x93, 0xa8);
pub const BIRINCI: Color32 = Color32::from_rgb(0xf3, 0xd6, 0x60);

// Can cubugunun koyu ucu.
pub const HP_BAS: Color32 = Color32::from_rgb(0xc9, 0x45, 0x45);

pub fn alfa(renk: Color32, oran: f32) -> Color32 {
    // Katman saydamligini ana rengi degistirmeden uygular.
    Color32::from_rgba_unmultiplied(
        renk.r(),
        renk.g(),
        renk.b(),
        (f32::from(renk.a()) * oran).round() as u8,
    )
}

pub fn beyaza(renk: Color32, oran: f32) -> Color32 {
    let karistir = |a: u8| (f32::from(a) * oran + 255.0 * (1.0 - oran)).round() as u8;
    Color32::from_rgb(karistir(renk.r()), karistir(renk.g()), karistir(renk.b()))
}
