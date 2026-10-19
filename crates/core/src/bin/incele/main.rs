//! Kayıt inceleme aracı: pcap'teki oyun akışını birleştirir, çerçeveleri yürütür, YALNIZ istatistik basar.
//! Kayıtlar oyuncu adı içerir: gövde hex'i yalnız izin listesindeki türler (hasar, periyodik, HP) için ve
//! `--hex` istenince basılır. Kullanım: incele [--dosya <pcap>] [--port <sunucu portu>] [--hex 0438 --adet 5]

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use canmeter_core::capture::akis::Akis;
use canmeter_core::capture::kaynak_pcap::PcapDosyasi;
use canmeter_core::capture::paket;
use canmeter_core::protocol::mesaj;
use canmeter_core::protocol::yuruyucu::Yuruyucu;

mod args;
mod beceri_izi;
mod boru_izi;
mod ozet;
mod savas_izi;
mod sinif;
mod sunucu;

const HEX_IZINLI: [[u8; 2]; 3] = [[0x04, 0x38], [0x05, 0x38], [0x1B, 0x92]];
/// Ad taşıyan türler: yalnız harf dizileri maskelenmiş hex basılır.
#[rustfmt::skip]
const HEX_MASKELI: [[u8; 2]; 5] = [[0x33, 0x36], [0x45, 0x36], [0x02, 0x97], [0x41, 0x36], [0x42, 0x36]];

fn main() {
    let args::Secenekler {
        dosya,
        port,
        sunucu: sunucu_secimi,
        boru,
        savas,
        beceri,
        hepsi,
        hex,
        adet,
        hatali,
    } = args::oku();
    if boru {
        boru_izi::calistir(dosya, hepsi);
        return;
    }
    if savas {
        savas_izi::calistir(dosya, hepsi, beceri);
        return;
    }
    let dosya = dosya
        .or_else(en_yeni_kayit)
        .expect("kayıt bulunamadı (--dosya ver)");
    let sunucu = sunucu_secimi
        .or_else(|| sunucu::sec(&dosya, port))
        .expect("oyun akışı bulunamadı");
    println!(
        "dosya: {}",
        dosya.file_name().unwrap_or_default().to_string_lossy()
    );
    println!("sunucu ucu: {sunucu}");
    let mut kaynak = PcapDosyasi::ac(&dosya).expect("pcap açılamadı");
    let (mut akis, mut yuruyucu) = (Akis::new(), Yuruyucu::new());
    let mut turler: ozet::Sayaclar = HashMap::new();
    let (mut segment, mut bayt, mut imza, mut basilan) = (0u64, 0u64, 0u64, 0usize);
    let mut siniflar = sinif::SinifSayaci::default();
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
            let e = turler.entry(tur).or_insert((0, 0, 0));
            e.0 += 1;
            e.1 += govde.len() as u64;
            let cozum = mesaj::coz(tur, govde);
            let cozuldu = cozum.is_some();
            e.2 += u64::from(cozuldu);
            if let Some(mesaj) = &cozum {
                siniflar.say(mesaj);
            }
            let maskeli = HEX_MASKELI.contains(&tur);
            let istenen =
                hex == Some(tur) && (HEX_IZINLI.contains(&tur) || maskeli) && (!hatali || !cozuldu);
            if istenen && basilan < adet {
                basilan += 1;
                println!(
                    "  {:02X}{:02X} [{}]: {}",
                    tur[0],
                    tur[1],
                    govde.len(),
                    if maskeli {
                        ozet::hex_maskeli(govde)
                    } else {
                        ozet::hex_yaz(govde)
                    }
                );
            }
        });
        akis.tuket(n);
    }
    ozet::yaz(segment, bayt, imza, akis.veri().len(), &turler);
    siniflar.yaz();
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
