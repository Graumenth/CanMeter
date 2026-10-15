//! Kayıt inceleme aracı: pcap'teki oyun akışını birleştirir, çerçeveleri yürütür, YALNIZ istatistik basar.
//! Kayıtlar oyuncu adı içerir: gövde hex'i yalnız izin listesindeki türler (hasar, periyodik, HP) için ve
//! `--hex` istenince basılır. Kullanım: incele [--dosya <pcap>] [--port <sunucu portu>] [--hex 0438 --adet 5]

use std::collections::HashMap;
use std::net::SocketAddrV4;
use std::path::{Path, PathBuf};

use canmeter_core::capture::akis::Akis;
use canmeter_core::capture::kaynak_pcap::PcapDosyasi;
use canmeter_core::capture::paket;
use canmeter_core::protocol::yuruyucu::Yuruyucu;

const HEX_IZINLI: [[u8; 2]; 3] = [[0x04, 0x38], [0x05, 0x38], [0x1B, 0x92]];

fn main() {
    let (dosya, port, hex, adet) = argumanlar();
    let dosya = dosya
        .or_else(en_yeni_kayit)
        .expect("kayıt bulunamadı (--dosya ver)");
    let sunucu = sunucu_sec(&dosya, port).expect("oyun akışı bulunamadı");
    println!(
        "dosya: {}",
        dosya.file_name().unwrap_or_default().to_string_lossy()
    );
    println!("sunucu ucu: {sunucu}");
    let mut kaynak = PcapDosyasi::ac(&dosya).expect("pcap açılamadı");
    let (mut akis, mut yuruyucu) = (Akis::new(), Yuruyucu::new());
    let mut turler: HashMap<[u8; 2], (u64, u64)> = HashMap::new();
    let (mut segment, mut bayt, mut imza, mut basilan) = (0u64, 0u64, 0u64, 0usize);
    while let Some(p) = kaynak.sonraki() {
        let Some(s) = paket::ayristir(p.veri).filter(|s| s.kaynak == sunucu && !s.yuk.is_empty())
        else {
            continue;
        };
        segment += 1;
        bayt += s.yuk.len() as u64;
        imza += s
            .yuk
            .windows(3)
            .filter(|w| *w == [0x0E, 0, 0x36] || *w == [0x06, 0, 0x36])
            .count() as u64;
        akis.ekle(s.seq, s.yuk);
        let n = yuruyucu.yurut(akis.veri(), &mut |tur, govde| {
            let e = turler.entry(tur).or_insert((0, 0));
            e.0 += 1;
            e.1 += govde.len() as u64;
            if hex == Some(tur) && basilan < adet && HEX_IZINLI.contains(&tur) {
                basilan += 1;
                println!(
                    "  {:02X}{:02X} [{}]: {}",
                    tur[0],
                    tur[1],
                    govde.len(),
                    hex_yaz(govde)
                );
            }
        });
        akis.tuket(n);
    }
    ozet(segment, bayt, imza, akis.veri().len(), &turler);
}

fn ozet(segment: u64, bayt: u64, imza: u64, kalan: usize, turler: &HashMap<[u8; 2], (u64, u64)>) {
    let mesaj: u64 = turler.values().map(|v| v.0).sum();
    println!(
        "segment={segment} gelen bayt={bayt} mesaj={mesaj} tür sayısı={} imza(0E/06 00 36)={imza} tüketilmeyen={kalan}",
        turler.len()
    );
    let mut sirali: Vec<_> = turler.iter().collect();
    sirali.sort_by_key(|(_, (adet, _))| std::cmp::Reverse(*adet));
    for (tur, (adet, bayt)) in sirali.iter().take(30) {
        println!(
            "  {:02X} {:02X}  adet={adet:<7} gövde bayt={bayt}",
            tur[0], tur[1]
        );
    }
    for tur in [
        [0x04, 0x38],
        [0x05, 0x38],
        [0x1B, 0x92],
        [0x33, 0x36],
        [0x44, 0x36],
        [0x45, 0x36],
        [0x02, 0x97],
    ] {
        let (adet, _) = turler.get(&tur).copied().unwrap_or((0, 0));
        println!("önemli {:02X} {:02X}: {adet}", tur[0], tur[1]);
    }
}

/// İlk geçiş: gönderen uç başına bayt; 443/80 dışı en çok veri gönderen uç oyun sunucusudur.
fn sunucu_sec(dosya: &Path, port: Option<u16>) -> Option<SocketAddrV4> {
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

fn en_yeni_kayit() -> Option<PathBuf> {
    let klasor = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../kayitlar");
    std::fs::read_dir(klasor)
        .ok()?
        .flatten()
        .map(|g| g.path())
        .filter(|p| p.extension().is_some_and(|u| u == "pcap"))
        .max_by_key(|p| p.metadata().and_then(|m| m.modified()).ok())
}

fn argumanlar() -> (Option<PathBuf>, Option<u16>, Option<[u8; 2]>, usize) {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let deger = |ad: &str| {
        a.iter()
            .position(|x| x == ad)
            .and_then(|i| a.get(i + 1))
            .cloned()
    };
    let hex = deger("--hex")
        .and_then(|h| u16::from_str_radix(&h, 16).ok())
        .map(u16::to_be_bytes);
    let adet = deger("--adet").and_then(|n| n.parse().ok()).unwrap_or(3);
    (
        deger("--dosya").map(PathBuf::from),
        deger("--port").and_then(|p| p.parse().ok()),
        hex,
        adet,
    )
}

fn hex_yaz(b: &[u8]) -> String {
    b.iter()
        .map(|x| format!("{x:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}
