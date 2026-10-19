//! Yakalama iş parçacığı: Npcap adaptörünü açar, TCP paketlerini zaman damgasıyla kanala aktarır.
//! Hiçbir paket gönderilmez; kanal kapanınca ya da yakalama hatasında döner.
use super::HamSegment;
use std::sync::mpsc::SyncSender;

pub(super) fn dongu(ad: String, gonderici: SyncSender<HamSegment>) {
    let Ok(mut yakalama) = pcap::Capture::from_device(ad.as_str()).and_then(|c| {
        c.promisc(false)
            .snaplen(65_535)
            .immediate_mode(true)
            .timeout(500)
            .open()
    }) else {
        return;
    };
    if yakalama.filter("tcp", true).is_err() {
        return;
    }
    loop {
        match yakalama.next_packet() {
            Ok(paket) => {
                let saniye = u64::try_from(paket.header.ts.tv_sec).unwrap_or(0);
                let mikro = u64::try_from(paket.header.ts.tv_usec).unwrap_or(0);
                let ham = HamSegment {
                    zaman_ms: saniye.saturating_mul(1_000) + mikro / 1_000,
                    veri: paket.data.to_vec(),
                };
                if gonderici.send(ham).is_err() {
                    break;
                }
            }
            Err(pcap::Error::TimeoutExpired) => {}
            Err(_) => break,
        }
    }
}
