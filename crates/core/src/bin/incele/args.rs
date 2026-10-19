//! Komut satırı: [--dosya <pcap>] [--port <sunucu portu>] [--sunucu <ip:port>] [--boru [--hepsi]] [--savas [--hepsi] [--beceri] [--arsiv <klasör>]] [--hex 0438 --adet 5 [--hatali]]

use std::net::SocketAddrV4;
use std::path::PathBuf;

pub struct Secenekler {
    pub dosya: Option<PathBuf>,
    pub port: Option<u16>,
    /// Verilirse uç seçimi yapılmaz (aynı kayıtta ikinci sunucu, 0.6.10).
    pub sunucu: Option<SocketAddrV4>,
    pub boru: bool,
    pub savas: bool,
    pub beceri: bool,
    pub arsiv: Option<PathBuf>,
    pub hepsi: bool,
    pub hex: Option<[u8; 2]>,
    pub adet: usize,
    /// `--hex` ile yalnız çözülemeyen gövdeleri bas.
    pub hatali: bool,
}

pub fn oku() -> Secenekler {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let deger = |ad: &str| {
        a.iter()
            .position(|x| x == ad)
            .and_then(|i| a.get(i + 1))
            .cloned()
    };
    Secenekler {
        dosya: deger("--dosya").map(PathBuf::from),
        port: deger("--port").and_then(|p| p.parse().ok()),
        sunucu: deger("--sunucu").and_then(|s| s.parse().ok()),
        boru: a.iter().any(|x| x == "--boru"),
        savas: a.iter().any(|x| x == "--savas"),
        beceri: a.iter().any(|x| x == "--beceri"),
        arsiv: deger("--arsiv").map(PathBuf::from),
        hepsi: a.iter().any(|x| x == "--hepsi"),
        hex: deger("--hex")
            .and_then(|h| u16::from_str_radix(&h, 16).ok())
            .map(u16::to_be_bytes),
        adet: deger("--adet").and_then(|n| n.parse().ok()).unwrap_or(3),
        hatali: a.iter().any(|x| x == "--hatali"),
    }
}
