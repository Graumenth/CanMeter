//! Oyun odaktayken de çalışan global kısayollar, tek yönetici: Ctrl+Shift+K kilit (0.4.x), Ctrl+Shift+R
//! yeniden başlat (0.8.0). Ana (olay döngüsü) thread'inde kurulur; Windows'ta gizli pencere bu thread'e bağlanır.
//! Kayıt başarısızsa kısayol yoktur, uygulama düğmelerle çalışmaya devam eder.

use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};

pub struct GlobalKisayol {
    /// Yönetici düşünce kayıt silinir.
    _yonetici: Option<GlobalHotKeyManager>,
    kilit: u32,
    yenile: u32,
}

/// Bu karede basılanlar.
#[derive(Clone, Copy, Default)]
pub struct Basilanlar {
    pub kilit: bool,
    pub yenile: bool,
}

impl GlobalKisayol {
    pub fn new() -> Self {
        let tuslar = Some(Modifiers::CONTROL | Modifiers::SHIFT);
        let kilit = HotKey::new(tuslar, Code::KeyK);
        let yenile = HotKey::new(tuslar, Code::KeyR);
        let yonetici = GlobalHotKeyManager::new()
            .and_then(|y| y.register_all(&[kilit, yenile]).map(|_| y))
            .map_err(|hata| eprintln!("Global kısayollar kaydedilemedi: {hata}"))
            .ok();
        Self {
            _yonetici: yonetici,
            kilit: kilit.id(),
            yenile: yenile.id(),
        }
    }

    /// Her karede bir kez: kanaldaki olayları tüketir.
    pub fn oku(&self) -> Basilanlar {
        let kanal = GlobalHotKeyEvent::receiver();
        let mut b = Basilanlar::default();
        while let Ok(olay) = kanal.try_recv() {
            if olay.state != HotKeyState::Pressed {
                continue;
            }
            b.kilit |= olay.id == self.kilit;
            b.yenile |= olay.id == self.yenile;
        }
        b
    }
}
