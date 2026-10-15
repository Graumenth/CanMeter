//! Pcap dosyasından çevrimdışı paket kaynağı; yalnız Ethernet dosyaları.

use std::path::Path;

pub struct Paket<'a> {
    /// Yakalama zamanı, mikrosaniye (epoch).
    pub zaman_us: u64,
    /// Ham Ethernet çerçevesi.
    pub veri: &'a [u8],
}

pub struct PcapDosyasi {
    yakalama: pcap::Capture<pcap::Offline>,
}

impl PcapDosyasi {
    /// Dosyayı açar; bağlantı türü Ethernet değilse hata verir.
    pub fn ac(yol: &Path) -> Result<Self, pcap::Error> {
        let yakalama = pcap::Capture::from_file(yol)?;
        if yakalama.get_datalink() != pcap::Linktype::ETHERNET {
            return Err(pcap::Error::InvalidLinktype);
        }
        Ok(Self { yakalama })
    }

    /// Sıradaki paket; dosya bitince ya da okumada hata olunca `None`.
    pub fn sonraki(&mut self) -> Option<Paket<'_>> {
        let paket = self.yakalama.next_packet().ok()?;
        let sec = u64::try_from(paket.header.ts.tv_sec).ok()?;
        let usec = u64::try_from(paket.header.ts.tv_usec).ok()?;
        Some(Paket {
            zaman_us: sec * 1_000_000 + usec,
            veri: paket.data,
        })
    }
}

#[cfg(test)]
mod testler {
    use super::*;
    use std::fs;
    fn kayit(v: &mut Vec<u8>, sec: u32, usec: u32, veri: &[u8]) {
        for n in [sec, usec, veri.len() as u32, veri.len() as u32] {
            v.extend(n.to_le_bytes());
        }
        v.extend(veri);
    }
    #[test]
    fn pcap_oku() {
        let kok = std::env::temp_dir().join(format!("canmeter-pcap-{}", std::process::id()));
        let girdi = kok.with_extension("girdi");
        let cikti = kok.with_extension("pcap");
        let mut ham = vec![
            0xd4, 0xc3, 0xb2, 0xa1, 2, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0,
        ];
        kayit(&mut ham, 1, 3, b"bir");
        kayit(&mut ham, 2, 4, b"iki");
        fs::write(&girdi, ham).unwrap();
        let mut kaynak = pcap::Capture::from_file(&girdi).unwrap();
        let olu = pcap::Capture::dead(pcap::Linktype::ETHERNET).unwrap();
        let mut yazici = olu.savefile(&cikti).unwrap();
        for _ in 0..2 {
            yazici.write(&kaynak.next_packet().unwrap());
        }
        yazici.flush().unwrap();
        drop(yazici);
        let mut oku = PcapDosyasi::ac(&cikti).unwrap();
        let ilk = oku.sonraki().unwrap();
        assert_eq!((ilk.zaman_us, ilk.veri), (1_000_003, b"bir" as &[u8]));
        let ikinci = oku.sonraki().unwrap();
        assert_eq!((ikinci.zaman_us, ikinci.veri), (2_000_004, b"iki" as &[u8]));
        assert!(oku.sonraki().is_none());
        drop(oku);
        drop(kaynak);
        fs::remove_file(girdi).unwrap();
        fs::remove_file(cikti).unwrap();
    }
}
