//! Paket yakalama katmanı: arayüz seçimi, BPF filtresi, TCP akış birleştirme.
//!
//! Bu katman yalnız OKUR. Hiçbir fonksiyon hat üzerine paket göndermez; bu sınır
//! CLAUDE.md'deki ürün sınırıdır ve kod incelemesinde denetlenir.

pub mod arayuz;
