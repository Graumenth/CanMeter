use super::kacis::kacis;
use crate::{bicim, hesap::KosuOzeti, oku::Gecmis};
use std::collections::BTreeMap;

pub fn secim(g: &Gecmis, kosular: &[KosuOzeti]) -> String {
    let haritalar: BTreeMap<_, _> = kosular
        .iter()
        .filter(|k| k.savas > 0)
        .map(|k| (k.harita, k.ad.as_str()))
        .collect();
    let mut haritalar: Vec<_> = haritalar.into_iter().collect();
    haritalar.sort_by_key(|(harita, ad)| (ad.to_lowercase(), *harita));
    let mut cikti = String::from(
        r#"<div class="cipler"><small>Bölge</small><select class="bolge-sec"><option value="hepsi">Hepsi</option>"#,
    );
    for (harita, ad) in haritalar {
        cikti.push_str(&format!(
            r#"<option value="{harita}">{}</option>"#,
            kacis(ad)
        ));
    }
    if g.savaslar.iter().any(|e| e.baslik.bolge.is_none()) {
        cikti.push_str(r#"<option value="yok">Bölge yok</option>"#);
    }
    cikti.push_str("</select></div>");
    cikti
}

pub fn baslik(harita: Option<u32>, ad: &str, giris: Option<&str>, savas: usize) -> String {
    let bolge = harita.map_or_else(|| "yok".into(), |h| h.to_string());
    let tarih = giris.map_or_else(String::new, |s| format!("{} · ", bicim::tarih(s, true)));
    format!(
        r#"<div class="kosu" data-bolge="{bolge}"><b>{}</b><span>{tarih}{savas} savaş</span></div>"#,
        kacis(ad)
    )
}
