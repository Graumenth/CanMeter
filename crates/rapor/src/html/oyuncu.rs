use super::{beceri, kacis::kacis, ortak::hucre};
use crate::{
    bicim, hesap,
    sema::{Encounter, Katilimci},
};

fn satir(savas: &Encounter, k: &Katilimci, sira: usize, azami: u64) -> String {
    let b = &savas.baslik;
    let sinif = k.sinif.to_ascii_lowercase();
    let ikon = if matches!(
        sinif.as_str(),
        "gladiator"
            | "templar"
            | "assassin"
            | "ranger"
            | "sorcerer"
            | "spiritmaster"
            | "cleric"
            | "chanter"
    ) {
        format!(r#"<img class="ikon" src="../ikon/{sinif}.png" alt="">"#)
    } else {
        String::new()
    };
    let ben = if k.ben {
        r#"<span class="ben">BEN</span>"#
    } else {
        ""
    };
    let mut cikti = format!(
        r#"<tr class="oyuncu">
<td class="sol soluk">{sira}</td>
<td class="sol">{ikon}<span class="ad">{}</span>{ben}<span class="sinif">{}</span></td>
<td class="sol" data-v="{}"><div class="cubuk"><i style="width:{:.2}%;background:{}"></i><b class="sayi">{}</b></div></td>"#,
        kacis(&k.ad),
        kacis(&k.sinif),
        k.hasar,
        hesap::oran(k.hasar as f64, azami as f64) * 100.0,
        hesap::renk(&k.sinif),
        bicim::sayi(k.hasar as f64)
    );
    let dps = hesap::dps(k.hasar, b.sure_ms);
    let hps = hesap::dps(k.iyilestirme, b.sure_ms);
    let pay = hesap::oran(k.hasar as f64, b.toplam_hasar as f64);
    let kritik = hesap::oran(f64::from(k.kritik), f64::from(k.vurus));
    for (metin, deger, ek) in [
        (bicim::sayi(dps), dps, ""),
        (bicim::yuzde(pay), pay, " soluk"),
        (bicim::yuzde(kritik), kritik, ""),
        (bicim::sayi(k.iyilestirme as f64), k.iyilestirme as f64, ""),
        (bicim::sayi(hps), hps, ""),
    ] {
        cikti.push_str(&hucre(&metin, deger, ek));
    }
    let stil = if k.olum > 0 {
        r#" style="color:#e5484d""#
    } else {
        ""
    };
    let yok = if k.olum == 0 { " yok" } else { "" };
    cikti.push_str(&format!(
        r#"<td class="sayi{yok}" data-v="{}"{stil}>{}</td></tr>"#,
        k.olum, k.olum
    ));
    cikti
}

pub fn yaz(savas: &Encounter) -> String {
    let mut cikti = String::from(
        r#"<section class="bolum"><h2>Oyuncular</h2><table>
<thead><tr><th class="sol">#</th><th class="sol">Oyuncu</th><th class="sol" data-sira>Hasar</th><th data-sira>DPS</th><th data-sira>Pay</th><th data-sira>Kritik</th><th data-sira>İyileştirme</th><th data-sira>HPS</th><th data-sira>Ölüm</th></tr></thead><tbody>"#,
    );
    let oyuncular = hesap::oyuncular(savas);
    let azami = oyuncular.first().map_or(0, |k| k.hasar);
    for (indeks, oyuncu) in oyuncular.iter().enumerate() {
        cikti.push_str(&satir(savas, oyuncu, indeks + 1, azami));
        cikti.push_str(&beceri::yaz(savas, oyuncu, indeks == 0));
    }
    cikti.push_str("</tbody></table></section>");
    cikti
}
