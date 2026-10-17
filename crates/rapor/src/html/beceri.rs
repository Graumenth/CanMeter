use super::{kacis::kacis, ortak::hucre};
use crate::{
    bicim, hesap,
    sema::{Beceri, Encounter, Katilimci},
};

fn nitelik(deger: Option<u32>, vurus: u32) -> String {
    let metin = match deger {
        None => "?".into(),
        Some(0) => "0".into(),
        Some(sayi) => bicim::yuzde(hesap::oran(f64::from(sayi), f64::from(vurus))),
    };
    hucre(&metin, deger.map_or(0.0, f64::from), "")
}

fn satir(b: &Beceri, k: &Katilimci, azami: u64) -> String {
    let iyilestirme = hesap::iyilestirme(b);
    let miktar = hesap::miktar(b);
    let toplam = if iyilestirme { k.iyilestirme } else { k.hasar };
    let renk = if iyilestirme {
        "#3fb950"
    } else {
        hesap::renk(&k.sinif)
    };
    let ek = if iyilestirme { " (iyileştirme)" } else { "" };
    let periyodik = if b.aile == "periyodik" {
        r#" <span class="soluk">(DoT)</span>"#
    } else {
        ""
    };
    let (ad, yok) = match &b.ad {
        Some(ad) if !ad.is_empty() => (kacis(ad), ""),
        _ => (format!("#{}", b.kimlik), " yok"),
    };
    let mut cikti = format!(
        r#"<tr class="beceri"><td class="sol{yok}">{ad}{ek}{periyodik}</td>
<td class="sol" data-v="{miktar}" title="{}"><div class="mini"><i style="width:{:.2}%;background:{renk}"></i></div></td>"#,
        miktar,
        hesap::oran(miktar as f64, azami as f64).min(1.0) * 100.0
    );
    cikti.push_str(&hucre(
        &bicim::yuzde(hesap::oran(miktar as f64, toplam as f64)),
        miktar as f64,
        "",
    ));
    cikti.push_str(&hucre(&b.vurus.to_string(), f64::from(b.vurus), ""));
    cikti.push_str(&hucre(&b.tick.to_string(), f64::from(b.tick), ""));
    let kritik = if b.tick > 0 && b.vurus == 0 {
        "?".into()
    } else {
        bicim::yuzde(hesap::oran(f64::from(b.kritik), f64::from(b.vurus)))
    };
    cikti.push_str(&hucre(&kritik, f64::from(b.kritik), ""));
    cikti.push_str(&nitelik(b.nitelik.perfect, b.vurus));
    cikti.push_str(&nitelik(b.nitelik.back, b.vurus));
    cikti.push_str(&hucre(
        &b.multi_vurus.to_string(),
        f64::from(b.multi_vurus),
        "",
    ));
    cikti.push_str(&hucre(
        &bicim::sayi(b.en_buyuk as f64),
        b.en_buyuk as f64,
        "",
    ));
    cikti.push_str("</tr>");
    cikti
}

pub fn yaz(savas: &Encounter, oyuncu: &Katilimci, ilk: bool) -> String {
    let acik = if ilk { " acik" } else { "" };
    let beceriler = hesap::beceriler(savas, oyuncu.no);
    if beceriler.is_empty() {
        return format!(r#"<tr class="kirilim{acik}"><td colspan="9"></td></tr>"#);
    }
    let azami = beceriler.first().map_or(0, |b| hesap::miktar(b));
    let mut cikti = format!(
        r#"<tr class="kirilim{acik}"><td colspan="9"><table>
<thead><tr><th class="sol">Beceri</th><th class="sol">Hasar</th><th>Pay</th><th>Vuruş</th><th>Tick</th><th>Kritik</th><th>Perfect</th><th>Back</th><th>Multi</th><th>En büyük</th></tr></thead><tbody>"#
    );
    for b in beceriler {
        cikti.push_str(&satir(b, oyuncu, azami));
    }
    cikti.push_str("</tbody></table></td></tr>");
    cikti
}
