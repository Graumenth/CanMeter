//! Savaş başına beceri kırılımı izi: kaynak başına en çok hasar veren 3 beceri, kova ve yaşam özeti. Ad basmaz.

use canmeter_core::stats::ayrinti::{Aile, Ayrinti, BeceriAnahtar, BeceriSayac, Olay};
use canmeter_core::stats::gecmis::Ozet;

pub fn yaz(ozet: &Ozet, ayrinti: &Ayrinti) {
    println!("{}", crate::savas_izi::ozet_satiri(ozet, ozet.baslangic_ms));
    let mut kaynaklar: Vec<_> = ayrinti
        .beceriler
        .keys()
        .map(|anahtar| anahtar.kaynak)
        .chain(ayrinti.kovalar.keys().copied())
        .collect();
    kaynaklar.sort_unstable();
    kaynaklar.dedup();
    for kaynak in kaynaklar {
        kaynak_yaz(kaynak, ayrinti);
    }
    println!("  yasam: {} olay", ayrinti.yasam.len());
    for olay in &ayrinti.yasam {
        let kaynak = kimlik(olay.kaynak);
        let beceri = kimlik(olay.beceri);
        let tur = match olay.olay {
            Olay::Olum => "Olum",
            Olay::Dirilis => "Dirilis",
        };
        println!(
            "    +{}ms kimlik={} {tur} kaynak={kaynak} beceri={beceri}",
            olay.ms, olay.kimlik
        );
    }
    if ayrinti.tasan > 0 {
        println!("  tasan={}", ayrinti.tasan);
    }
}

fn kaynak_yaz(kaynak: u32, ayrinti: &Ayrinti) {
    let (adet, genislik, tepe) = ayrinti.kovalar.get(&kaynak).map_or((0, 0, 0), |kova| {
        (
            kova.hasar.len(),
            kova.genislik_ms,
            kova.hasar.iter().copied().fold(0, u32::max),
        )
    });
    println!(
        "  kaynak={kaynak} hasar={} iyi={} kova={adet}x{genislik}ms tepe={tepe}",
        ayrinti.toplam_hasar(kaynak),
        ayrinti.toplam_iyilestirme(kaynak)
    );
    let mut beceriler: Vec<_> = ayrinti
        .beceriler
        .iter()
        .filter(|(anahtar, _)| anahtar.kaynak == kaynak)
        .collect();
    beceriler.sort_unstable_by(|(a, sa), (b, sb)| {
        sb.hasar
            .cmp(&sa.hasar)
            .then_with(|| a.beceri.cmp(&b.beceri))
            .then_with(|| aile_sira(a.aile).cmp(&aile_sira(b.aile)))
    });
    for (anahtar, sayac) in beceriler.into_iter().take(3) {
        beceri_yaz(anahtar, sayac);
    }
}

fn beceri_yaz(anahtar: &BeceriAnahtar, sayac: &BeceriSayac) {
    let aile = match anahtar.aile {
        Aile::Dogrudan => "Dogrudan",
        Aile::Periyodik => "Periyodik",
    };
    println!(
        "    beceri={} aile={aile} hasar={} vurus={} tick={} kritik={} kacinma={} multi={} enbuyuk={} kapsam={} perfect={} block={} back={}",
        anahtar.beceri,
        sayac.hasar,
        sayac.vurus,
        sayac.tick,
        sayac.kritik,
        sayac.kacinma,
        sayac.multi_vurus,
        sayac.en_buyuk,
        sayac.kapsam,
        sayac.perfect,
        sayac.block,
        sayac.back
    );
}

fn aile_sira(aile: Aile) -> u8 {
    match aile {
        Aile::Dogrudan => 0,
        Aile::Periyodik => 1,
    }
}

fn kimlik(deger: Option<u32>) -> String {
    deger.map_or_else(|| "-".to_owned(), |kimlik| kimlik.to_string())
}
