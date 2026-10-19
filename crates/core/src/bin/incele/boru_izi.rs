//! Boru zaman çizelgesi: kayıt Boru'dan geçer; sunucu kilidi, geçiş, hazırlık durumu ya da bölge değişince satır basar.
//! Oyuncu adı basmaz; bölge satırında yalnız harita adı yer alır.

use std::collections::HashMap;
use std::net::SocketAddrV4;
use std::path::{Path, PathBuf};

use canmeter_core::capture::kaynak_pcap::PcapDosyasi;
use canmeter_core::capture::paket::ayristir;
use canmeter_core::kaynak::boru::Boru;
use canmeter_sema::Haritalar;

pub fn calistir(dosya: Option<PathBuf>, hepsi: bool) {
    if hepsi {
        let klasor = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../kayitlar");
        let Ok(girdiler) = std::fs::read_dir(klasor) else {
            eprintln!("kayıtlar klasörü okunamadı");
            std::process::exit(1);
        };
        let mut dosyalar: Vec<_> = girdiler
            .flatten()
            .map(|girdi| girdi.path())
            .filter(|yol| yol.extension().is_some_and(|uzanti| uzanti == "pcap"))
            .collect();
        dosyalar.sort();
        for (sira, yol) in dosyalar.iter().enumerate() {
            if sira > 0 {
                println!();
            }
            let ad = yol.file_name().map_or_else(
                || yol.as_os_str().to_string_lossy(),
                |ad| ad.to_string_lossy(),
            );
            println!("dosya: {ad}");
            yaz(yol);
        }
        return;
    }
    let Some(yol) = dosya.or_else(crate::en_yeni_kayit) else {
        eprintln!("kayıt bulunamadı (--dosya ver)");
        std::process::exit(1);
    };
    let ad = yol.file_name().map_or_else(
        || yol.as_os_str().to_string_lossy(),
        |ad| ad.to_string_lossy(),
    );
    println!("dosya: {ad}");
    yaz(&yol);
}

pub fn yaz(dosya: &Path) {
    let mut kaynak = PcapDosyasi::ac(dosya).expect("pcap açılamadı");
    let mut boru = Boru::new();
    let haritalar = Haritalar::yerlesik();
    let izi = |b: &Boru| {
        (
            b.sunucu(),
            b.nesil(),
            b.durum(),
            b.bolge().map(|k| (k.harita, k.ornek)),
        )
    };
    let mut onceki = izi(&boru);
    let mut t0 = None;
    let mut son_ms = 0;
    let mut paket = 0u64;
    let mut segment = 0u64;
    // Uç başına (ilk ms, son ms, dolu segment, imzalı segment): geçişin neden olmadığını gösterir.
    let mut ucler: HashMap<SocketAddrV4, (u64, u64, u64, u64)> = HashMap::new();

    while let Some(p) = kaynak.sonraki() {
        paket += 1;
        let baslangic = *t0.get_or_insert(p.zaman_us);
        let zaman_ms = p.zaman_us.saturating_sub(baslangic) / 1000;
        son_ms = zaman_ms;
        let Some(s) = ayristir(p.veri) else {
            continue;
        };
        if !s.yuk.is_empty() {
            segment += 1;
            let e = ucler.entry(s.kaynak).or_insert((zaman_ms, zaman_ms, 0, 0));
            e.1 = zaman_ms;
            e.2 += 1;
            e.3 += u64::from(matches!(s.yuk, [0x0e | 0x06, 0x00, 0x36, ..]));
        }
        boru.segment(zaman_ms, &s);
        let simdiki = izi(&boru);
        if simdiki != onceki {
            let uc = simdiki.0.map_or_else(|| "-".to_owned(), |u| u.to_string());
            let bolge = simdiki.3.map_or_else(
                || "-".to_owned(),
                |(harita, ornek)| format!("{harita}/{ornek} {}", haritalar.goruntu(harita)),
            );
            println!(
                "+{:.3}s nesil={} uc={} durum={:?} bolge={bolge}",
                zaman_ms as f64 / 1000.0,
                simdiki.1,
                uc,
                simdiki.2
            );
            onceki = simdiki;
        }
    }
    println!(
        "son: +{:.3}s, {paket} paket, {segment} segment",
        son_ms as f64 / 1000.0
    );
    let mut liste: Vec<_> = ucler
        .into_iter()
        .filter(|(_, (_, _, n, imza))| *imza > 0 || *n >= 100)
        .collect();
    liste.sort_by_key(|(_, (ilk, ..))| *ilk);
    for (uc, (ilk, son, n, imza)) in liste {
        println!(
            "  uc={uc} ilk=+{:.3}s son=+{:.3}s segment={n} imzali={imza}",
            ilk as f64 / 1000.0,
            son as f64 / 1000.0
        );
    }
}
