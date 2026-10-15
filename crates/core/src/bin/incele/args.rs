//! Komut satırı: [--dosya <pcap>] [--port <sunucu portu>] [--hex 0438 --adet 5 [--hatali]]

use std::path::PathBuf;

pub struct Secenekler {
    pub dosya: Option<PathBuf>,
    pub port: Option<u16>,
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
        hex: deger("--hex")
            .and_then(|h| u16::from_str_radix(&h, 16).ok())
            .map(u16::to_be_bytes),
        adet: deger("--adet").and_then(|n| n.parse().ok()).unwrap_or(3),
        hatali: a.iter().any(|x| x == "--hatali"),
    }
}
