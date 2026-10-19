//! Savaş özeti izi: kayıt Boru'dan geçer; kapanan her savaş bir satır. Ad basmaz (hedef yalnız kimlik).

use std::path::{Path, PathBuf};

use canmeter_core::capture::kaynak_pcap::PcapDosyasi;
use canmeter_core::capture::paket::ayristir;
use canmeter_core::kaynak::boru::Boru;
use canmeter_core::stats::gecmis::{Neden, Ozet};

pub fn calistir(dosya: Option<PathBuf>, hepsi: bool, beceri: bool) {
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
            dosya_yaz(yol, beceri);
        }
        return;
    }
    let Some(yol) = dosya.or_else(crate::en_yeni_kayit) else {
        eprintln!("kayıt bulunamadı (--dosya ver)");
        std::process::exit(1);
    };
    dosya_yaz(&yol, beceri);
}

fn dosya_yaz(yol: &Path, beceri: bool) {
    let ad = yol.file_name().map_or_else(
        || yol.as_os_str().to_string_lossy(),
        |ad| ad.to_string_lossy(),
    );
    println!("dosya: {ad}");
    yaz(yol, beceri);
}

pub fn yaz(dosya: &Path, beceri: bool) {
    let mut kaynak = PcapDosyasi::ac(dosya).expect("pcap açılamadı");
    let mut boru = Boru::new();
    let mut t0_ms = None;
    let mut son_ms = 0;
    let mut son_dilim = None;
    let mut gecmis_uzunlugu = 0;
    while let Some(paket) = kaynak.sonraki() {
        let zaman_ms = paket.zaman_us / 1_000;
        t0_ms.get_or_insert(zaman_ms);
        son_ms = zaman_ms;
        if let Some(segment) = ayristir(paket.veri) {
            boru.segment(zaman_ms, &segment);
        }
        let dilim = zaman_ms / 250;
        if son_dilim != Some(dilim) {
            let _ = boru.anlik(zaman_ms);
            son_dilim = Some(dilim);
        }
        if beceri
            && boru.gecmis().len() > gecmis_uzunlugu
            && let Some(ozet) = boru.gecmis().last()
        {
            crate::beceri_izi::yaz(ozet, boru.ayrinti());
        }
        gecmis_uzunlugu = boru.gecmis().len();
    }
    boru.kapat(Neden::KaynakSonu, son_ms);
    if beceri
        && boru.gecmis().len() > gecmis_uzunlugu
        && let Some(ozet) = boru.gecmis().last()
    {
        crate::beceri_izi::yaz(ozet, boru.ayrinti());
    }
    let Some(t0_ms) = t0_ms else {
        println!("toplam: {} savaş", boru.gecmis().len());
        return;
    };
    if !beceri {
        for ozet in boru.gecmis() {
            println!("{}", ozet_satiri(ozet, t0_ms));
        }
    }
    println!("toplam: {} savaş", boru.gecmis().len());
}

pub fn ozet_satiri(ozet: &Ozet, t0_ms: u64) -> String {
    let hedef = ozet
        .hedef
        .as_ref()
        .map_or_else(|| "-".to_owned(), |hedef| hedef.kimlik.to_string());
    let hp = ozet.hedef.as_ref().map_or(0, |hedef| hedef.azami_hp);
    let baslangic = ozet.baslangic_ms.saturating_sub(t0_ms) as f64 / 1_000.0;
    let sure = ozet.sure_ms as f64 / 1_000.0;
    format!(
        "#{} +{baslangic:.1}s sure={sure:.1}s neden={:?} sonuc={:?} etiket={:?} hedef={hedef} hp={hp} hasar={} iyi={} oyuncu={} olum={}",
        ozet.kimlik,
        ozet.neden,
        ozet.sonuc,
        ozet.etiket,
        ozet.toplam_hasar,
        ozet.toplam_iyilestirme,
        ozet.katilimci,
        ozet.olum
    )
}
