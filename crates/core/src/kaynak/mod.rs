//! Anlık kaynağı (Strategy): savaş durumunu kimin ürettiğini soyutlar.
//!
//! `demo` oyunsuz UI geliştirme için; `canli` (faz 01 görev 6-7) yakalama boru hattını saracak.
//! Kaynak kendi saat tutmaz, zamanı `gecen` ile dışarıdan alır: deterministik test edilir ve
//! kimseyi boşuna uyandırmaz.

use std::time::Duration;

use crate::stats::SavasAnlik;

pub mod boru;
pub mod canli;
pub mod demo;
pub mod kayit;

/// Kaynağın hazırlık durumu; overlay hazır değilken kullanıcıya tek satır ipucu gösterir.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Durum {
    Hazir,
    /// Oyun sunucusu akışı henüz kilitlenmedi (oyun kapalı ya da trafik yok).
    SunucuYok,
    /// Akış var ama yerel oyuncu (33 36) görülmedi: CanMeter oyuna girdikten sonra açıldı.
    KarakterYok,
}

pub trait AnlikKaynagi: Send {
    /// Zamanı ilerletir. Durum değiştiyse yeni anlık döner; değişmediyse `None` (UI boşuna çizmez).
    fn ilerle(&mut self, gecen: Duration) -> Option<SavasAnlik>;
    /// Hazırlık durumu; demo gibi her zaman hazır kaynaklar varsayılanı kullanır.
    fn durum(&self) -> Durum {
        Durum::Hazir
    }
}
