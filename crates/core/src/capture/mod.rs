//! Paket yakalama katmanı: arayüz seçimi, BPF filtresi, TCP akış birleştirme.
//!
//! Bu katman yalnız OKUR. Hiçbir fonksiyon hat üzerine paket göndermez; bu sınır
//! CLAUDE.md'deki ürün sınırıdır ve kod incelemesinde denetlenir.

pub mod akis;
pub mod arayuz;
pub mod kaynak_pcap;
pub mod paket;
