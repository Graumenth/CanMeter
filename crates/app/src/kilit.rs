//! Overlay kilidi: kilitliyken pencere fare olaylarını oyuna geçirir (MousePassthrough), bu yüzden
//! kilit düğmesine tıklanamaz. Açmak için oyun odaktayken de çalışan global kısayol: Ctrl+Shift+K.

use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};

pub struct Kilit {
    pub kilitli: bool,
    /// Yönetici düşünce kayıt silinir; `None` ise kısayol yok (kayıt başarısız, kilit yine düğmeden kapanır).
    _yonetici: Option<GlobalHotKeyManager>,
    kisayol_id: u32,
}

impl Kilit {
    /// Ana (olay döngüsü) thread'inde çağrılır; Windows'ta gizli pencere bu thread'e bağlanır.
    pub fn new() -> Self {
        let kisayol = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyK);
        let yonetici = GlobalHotKeyManager::new()
            .and_then(|y| y.register(kisayol).map(|_| y))
            .map_err(|hata| eprintln!("Kilit kısayolu kaydedilemedi: {hata}"))
            .ok();
        Self {
            kilitli: false,
            _yonetici: yonetici,
            kisayol_id: kisayol.id(),
        }
    }

    /// Her karede: kısayola basıldıysa ya da skin düğmesi istediyse kilidi çevirir ve pencereye uygular.
    pub fn kare(&mut self, ctx: &egui::Context, dugme: bool) {
        let kanal = GlobalHotKeyEvent::receiver();
        let mut basildi = false;
        while let Ok(olay) = kanal.try_recv() {
            basildi |= olay.id == self.kisayol_id && olay.state == HotKeyState::Pressed;
        }
        if dugme || basildi {
            self.kilitli = !self.kilitli;
            ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(self.kilitli));
        }
    }
}
