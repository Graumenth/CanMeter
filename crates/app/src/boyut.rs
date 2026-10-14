//! Pencere boyutu: genişlik kullanıcıdan (kenardan sürükleme), yükseklik içerikten.
//! Kullanıcı genişliği değiştirince overlay bütünüyle ölçeklenir (egui zoom), yerleşim hep 440 nokta kalır.

/// Yerleşim genişliği (nokta); mockup'lar 440 px.
pub const TABAN_GENISLIK: f32 = 440.0;
const OLCEK_ARALIGI: std::ops::RangeInclusive<f32> = 0.5..=3.0;
const TOLERANS_PX: f32 = 1.5;

#[derive(Default)]
pub struct Boyut {
    /// Pencereye en son istenen fiziksel boyut (piksel). `None`: henüz istenmedi.
    hedef_px: Option<(f32, f32)>,
    /// Önceki karede görülen genişlik (piksel). Zoom değişince egui bir kare eski noktayla rapor
    /// eder ve sürükleme sırasında genişlik her kare değişir; kullanıcı değişikliği ancak iki
    /// ardışık karede aynı genişlik görülünce kabul edilir.
    onceki_genislik_px: f32,
}

impl Boyut {
    /// Her karede çağrılır. Kullanıcı genişliği değiştirdiyse `olcek` güncellenir ve `true` döner;
    /// yoksa pencere (440 x `icerik_nokta`) nokta boyutuna getirilir.
    pub fn kare(&mut self, ctx: &egui::Context, icerik_nokta: f32, olcek: &mut f32) -> bool {
        let dogal = ctx.native_pixels_per_point().unwrap_or(1.0);
        let ppp = ctx.pixels_per_point();
        let Some(pencere) = ctx.input(|i| i.viewport().inner_rect) else {
            return false;
        };
        let simdiki = (pencere.width() * ppp, pencere.height() * ppp);
        let kararli = (simdiki.0 - self.onceki_genislik_px).abs() <= TOLERANS_PX;
        self.onceki_genislik_px = simdiki.0;

        if let Some((hedef_w, _)) = self.hedef_px
            && (simdiki.0 - hedef_w).abs() > TOLERANS_PX
        {
            if !kararli {
                return false; // Zoom gecikmesi ya da sürükleme sürüyor: bir kare bekle.
            }
            let yeni = (simdiki.0 / (TABAN_GENISLIK * dogal))
                .clamp(*OLCEK_ARALIGI.start(), *OLCEK_ARALIGI.end());
            if (yeni - *olcek).abs() > 0.002 {
                *olcek = yeni;
                ctx.set_zoom_factor(yeni);
                self.hedef_px = Some((
                    TABAN_GENISLIK * dogal * yeni,
                    icerik_nokta.ceil() * dogal * yeni,
                ));
                return true;
            }
            // Ölçek sınırda: pencere aşağıda istenen boyuta çekilir.
        }

        let istenen = (TABAN_GENISLIK * ppp, icerik_nokta.ceil() * ppp);
        let farkli = |a: (f32, f32), b: (f32, f32)| {
            (a.0 - b.0).abs() > TOLERANS_PX || (a.1 - b.1).abs() > TOLERANS_PX
        };
        if self.hedef_px.is_none_or(|h| farkli(h, istenen)) || farkli(simdiki, istenen) {
            self.hedef_px = Some(istenen);
            let boyut = egui::vec2(TABAN_GENISLIK, icerik_nokta.ceil());
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(boyut));
        }
        false
    }
}
