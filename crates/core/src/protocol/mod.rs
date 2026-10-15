//! Oyun akışı protokolü: varint, dış çerçeve, LZ4 bundle, mesaj yürütme.
//! Gerçekler `CanMeterCC/protokol.md` (§1-3, §9). GPL koddan tek satır ya da test vektörü kopyalanmaz;
//! testler kendi ürettiğimiz baytlarla ve sahibin kaydından çıkarılan fixture'larla yazılır.
//!
//! Boru hattı: `capture::akis::Akis::veri()` -> [`yuruyucu::Yuruyucu::yurut`] -> (tür, gövde) -> `mesaj` (faz 01 görev 6).

pub mod cerceve;
pub mod lz4;
pub mod varint;
pub mod yuruyucu;
