//! Mockup CSS değişkenlerinden gelen renkler.

use egui::Color32;

pub const ZEMIN: Color32 = Color32::from_rgb(0x0b, 0x0e, 0x14);
pub const YAZI: Color32 = Color32::from_rgb(0xe9, 0xee, 0xf7);
pub const IKINCIL: Color32 = Color32::from_rgb(0x9a, 0xa4, 0xb6);
pub const SOLUK: Color32 = Color32::from_rgb(0x5e, 0x67, 0x79);
pub const AMBER: Color32 = Color32::from_rgb(0xf0, 0xb1, 0x3e);
pub const HP: Color32 = Color32::from_rgb(0xdd, 0x4a, 0x37);
pub const HP_KENAR: Color32 = Color32::from_rgb(0xff, 0x88, 0x68);
pub const CERCEVE: Color32 = Color32::from_rgba_unmultiplied_const(255, 255, 255, 23);
pub const CIZGI: Color32 = Color32::from_rgba_unmultiplied_const(255, 255, 255, 18);
pub const OLU: Color32 = Color32::from_rgb(0x7d, 0x86, 0x98);

pub fn saydam(renk: Color32, alfa: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(renk.r(), renk.g(), renk.b(), alfa)
}
