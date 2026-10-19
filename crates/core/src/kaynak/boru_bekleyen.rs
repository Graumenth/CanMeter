//! Geçiş bekleyen yabancı ucun tamponu: segment yükleri zamanlarıyla saklanır.
use std::net::SocketAddrV4;

/// Bekleyen uç ve segmentleri.
#[derive(Debug, Default)]
pub(super) struct Bekleyen {
    pub(super) uc: Option<SocketAddrV4>,
    /// Segment yükleri art arda; parcalar (seq, ms, bas, boy) ile adreslenir.
    pub(super) veri: Vec<u8>,
    pub(super) parcalar: Vec<(u32, u64, u32, u32)>,
    pub(super) son_imza_ms: u64,
    /// Tavan sıfırlanınca ilk imzalı segmente kadar imzasız yük saklanmaz.
    pub(super) taban_bekliyor: bool,
}
