//! Skin'ler kilit düğmesinin yerini her kare bildirir; kabuk kilitliyken yalnız o alanı tıklanabilir bırakır.
//! Skin sözleşmesi (`skin/mod.rs`) değişmez: alan egui'nin geçici deposuna yazılır, kabuk aynı karede okur.

use egui::{Id, Rect};

fn anahtar() -> Id {
    Id::new("canmeter_kilit_alani")
}

/// Skin başlığı çağırır; `alan` ui koordinatında (nokta).
pub fn bildir(ui: &egui::Ui, alan: Rect) {
    ui.ctx().data_mut(|d| d.insert_temp(anahtar(), alan));
}

/// Bu karede bildirilen alan; skin bildirmediyse `None` (kilit yalnız kısayolla açılır).
pub fn al(ctx: &egui::Context) -> Option<Rect> {
    ctx.data(|d| d.get_temp(anahtar()))
}
