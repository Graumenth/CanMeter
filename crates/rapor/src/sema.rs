//! Encounter dosya şeması v1: ortak tanım `canmeter-sema` crate'inde (meter yazar, rapor okur); burada yalnız
//! bir dosyanın birleştirilmiş hali.
pub use canmeter_sema::*;

/// Bir dosyanın birleştirilmiş hali; `oku.rs` doldurur.
#[derive(Debug, Clone, Default)]
pub struct Encounter {
    pub baslik: Baslik,
    pub katilimcilar: Vec<Katilimci>,
    pub beceriler: Vec<Beceri>,
    pub kovalar: Vec<Kova>,
    pub yasam: Vec<Yasam>,
    pub tamam: bool,
    pub dosya: String,
}
