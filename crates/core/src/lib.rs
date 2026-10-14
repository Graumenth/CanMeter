//! CanMeter çekirdeği.
//!
//! Saf kütüphane: UI'ı görmez, sunucuya paket göndermez, oyun belleğine dokunmaz.
//! Modüller faz 01'de dolar: `capture`, `protocol`, `combat`, `stats`, `replay` (bkz. CLAUDE.md).

pub mod capture;

/// Çekirdek sürümü; `Cargo.toml`'dan okunur, elle yazılmaz.
pub fn surum() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surum_bos_degil() {
        assert!(!surum().is_empty());
    }
}
