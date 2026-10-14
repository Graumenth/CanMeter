//! Pencere konumu: kullanıcı overlay'i nereye taşıdıysa ayara yazılır, sonraki açılış orada olur.
//! Konum doğal (zoom'suz) nokta cinsinden tutulur; `ViewportBuilder::with_position` da bunu bekler.

#[derive(Default)]
pub struct Konum;

impl Konum {
    /// Her karede: pencerenin dış köşesi ayardakinden farklıysa ayarı günceller ve `true` döner
    /// (kabuk gecikmeli kaydeder). Konum bilinmiyorsa (ilk kare) `false`.
    pub fn kare(&mut self, ctx: &egui::Context, ayar: &mut Option<[i32; 2]>) -> bool {
        let Some(dis) = ctx.input(|i| i.viewport().outer_rect) else {
            return false;
        };
        let dogal = ctx.native_pixels_per_point().unwrap_or(1.0);
        let carpan = ctx.pixels_per_point() / dogal;
        let simdiki = [
            (dis.min.x * carpan).round() as i32,
            (dis.min.y * carpan).round() as i32,
        ];
        if *ayar == Some(simdiki) {
            return false;
        }
        *ayar = Some(simdiki);
        true
    }
}
