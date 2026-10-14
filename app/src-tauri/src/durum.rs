//! Uygulamanın paylaşılan çalışma durumu.
//! Motor kilidi yayın thread'i ile komutlar arasında paylaşılır.

use std::sync::Mutex;

use canmeter_core::Motor;

/// Tauri uygulamasının paylaşılan savaş durumu.
pub struct Durum {
    pub motor: Mutex<Motor>,
}

impl Durum {
    /// Geliştirme ve boşta çalışma için sabit demo kaynağı.
    pub fn demo() -> Self {
        Self {
            motor: Mutex::new(Motor::demo(7)),
        }
    }
}
