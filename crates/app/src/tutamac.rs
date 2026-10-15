//! Boyutlandırma tutamacı: sağ kenar şeridi. Çerçevesiz winit penceresinde Windows kenar tutmaz
//! (WM_NCHITTEST her noktada istemci alanı döndürür); şerit egui'de yakalanır, pencereye BeginResize verilir
//! ve Windows'un kendi boyutlandırma döngüsü çalışır. Genişlik ölçeğe döner, yükseklik içerikten (boyut.rs).

use egui::{Color32, CursorIcon, Pos2, Rect, Sense, Stroke, Vec2, ViewportCommand};

/// Şerit genişliği (nokta); skin'lerin sağ kenar boşluğundan dar.
const SERIT: f32 = 6.0;
/// Ortadaki çizginin boyu (nokta).
const CIZGI_BOYU: f32 = 28.0;
/// Kilit düğmesi bildirilmediyse şerit başlığın altından başlar.
const BASLIK_PAYI: f32 = 60.0;

/// Skin çizildikten sonra, aynı `ui` içinde çağrılır. `icerik`: skin'in kapladığı alan.
pub fn ciz(ui: &mut egui::Ui, icerik: Rect, kilitli: bool) {
    if kilitli {
        return;
    }
    // Başlık düğmeleriyle çakışmasın: kilit düğmesinin altından başlar.
    let ust =
        crate::kilit_alani::al(ui.ctx()).map_or(icerik.top() + BASLIK_PAYI, |k| k.bottom() + 6.0);
    let alan = Rect::from_min_max(
        Pos2::new(icerik.right() - SERIT, ust),
        icerik.right_bottom(),
    );
    if alan.height() < CIZGI_BOYU {
        return;
    }
    let yanit = ui
        .interact(alan, ui.id().with("tutamac"), Sense::drag())
        .on_hover_cursor(CursorIcon::ResizeHorizontal);
    if yanit.drag_started() {
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            egui::viewport::ResizeDirection::East,
        ));
    }
    let renk = if yanit.hovered() {
        Color32::from_white_alpha(170)
    } else {
        Color32::from_white_alpha(60)
    };
    let orta = alan.center();
    let yarim = Vec2::new(0.0, CIZGI_BOYU / 2.0);
    ui.painter()
        .line_segment([orta - yarim, orta + yarim], Stroke::new(2.0, renk));
}
