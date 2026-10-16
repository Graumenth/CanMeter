//! Boyutlandırma tutamacı: sağ alt köşe (sahip isteği, 0.6.1; önce sağ kenar şeridiydi). Çerçevesiz winit
//! penceresinde Windows kenar tutmaz (WM_NCHITTEST hep istemci alanı); köşe egui'de yakalanır, pencereye
//! BeginResize verilir ve Windows'un kendi boyutlandırma döngüsü çalışır. Genişlik ölçeğe döner, yükseklik
//! içerikten (boyut.rs), bu yüzden çapraz sürüklemede yalnız genişlik kalıcıdır.

use egui::{Color32, CursorIcon, Rect, Sense, Stroke, Vec2, ViewportCommand};

/// Tutma alanı (nokta): köşeden içeri doğru kare.
const ALAN: f32 = 18.0;
/// Çizgiler köşeden bu uzaklıklarda, 45 derece.
const CIZGILER: [f32; 3] = [4.0, 8.0, 12.0];

/// Skin çizildikten sonra, aynı `ui` içinde çağrılır. `icerik`: skin'in kapladığı alan.
pub fn ciz(ui: &mut egui::Ui, icerik: Rect, kilitli: bool) {
    if kilitli {
        return;
    }
    let kose = icerik.right_bottom();
    let alan = Rect::from_min_max(kose - Vec2::splat(ALAN), kose);
    let yanit = ui
        .interact(alan, ui.id().with("tutamac"), Sense::drag())
        .on_hover_cursor(CursorIcon::ResizeNwSe);
    if yanit.drag_started() {
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            egui::viewport::ResizeDirection::SouthEast,
        ));
    }
    let renk = if yanit.hovered() {
        Color32::from_white_alpha(170)
    } else {
        Color32::from_white_alpha(70)
    };
    // Köşeye 3 nokta pay; çizgiler köşeden sola ve yukarı eşit uzaklıkta (45 derece).
    let kose = kose - Vec2::splat(3.0);
    for u in CIZGILER {
        ui.painter().line_segment(
            [kose - Vec2::new(u, 0.0), kose - Vec2::new(0.0, u)],
            Stroke::new(1.0, renk),
        );
    }
}
