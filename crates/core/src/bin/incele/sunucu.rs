//! Oyun sunucusu ucunu seçer (ilk geçiş).
use std::collections::HashMap;
use std::net::SocketAddrV4;
use std::path::Path;

use canmeter_core::capture::kaynak_pcap::PcapDosyasi;
use canmeter_core::capture::paket;

/// İlk geçiş: gönderen uç başına bayt; 443/80 dışı en çok veri gönderen uç oyun sunucusudur.
pub fn sec(dosya: &Path, port: Option<u16>) -> Option<SocketAddrV4> {
    let mut kaynak = PcapDosyasi::ac(dosya).ok()?;
    let mut toplam: HashMap<SocketAddrV4, u64> = HashMap::new();
    while let Some(p) = kaynak.sonraki() {
        if let Some(s) = paket::ayristir(p.veri) {
            *toplam.entry(s.kaynak).or_default() += s.yuk.len() as u64;
        }
    }
    toplam
        .into_iter()
        .filter(|(uc, _)| port.is_none_or(|p| uc.port() == p))
        .filter(|(uc, _)| ![443, 80].contains(&uc.port()))
        .max_by_key(|(_, b)| *b)
        .map(|(uc, _)| uc)
}
