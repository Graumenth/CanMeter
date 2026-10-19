use super::{kacis::kacis, ortak::hucre};
use crate::{bicim, hesap, oku::Gecmis};
use chrono::DateTime;
use std::collections::BTreeMap;

fn tablo(ad: &str, sutunlar: &[&str]) -> String {
    let mut cikti = format!(r#"<section class="bolum"><h2>{ad}</h2><table><thead><tr>"#);
    for sutun in sutunlar {
        cikti.push_str(&format!("<th data-sira>{sutun}</th>"));
    }
    cikti.push_str("</tr></thead><tbody>");
    cikti
}

fn son(bos: bool) -> &'static str {
    if bos {
        r#"</tbody></table><p class="soluk">Henüz bölge kaydı yok.</p></section>"#
    } else {
        "</tbody></table></section>"
    }
}

#[derive(Default)]
struct Toplam {
    ad: String,
    kosu: usize,
    savas: usize,
    zafer: usize,
    sure: u64,
}

pub fn yaz(g: &Gecmis) -> String {
    let kosular = hesap::kosular(g);
    let mut cikti = tablo(
        "Zindanlar",
        &[
            "Bölge",
            "Giriş",
            "Süre",
            "Savaş",
            "Zafer",
            "Wipe",
            "En iyi DPS",
        ],
    );
    let mut bolgeler = BTreeMap::<Option<u32>, Toplam>::new();
    for k in kosular {
        let giris = DateTime::parse_from_rfc3339(&k.giris_utc).map_or(0, |t| t.timestamp_millis());
        cikti.push_str(&format!(
            r#"<tr><td class="sol">{}</td><td data-v="{giris}">{}</td>"#,
            kacis(&k.ad),
            bicim::tarih(&k.giris_utc, false)
        ));
        for (metin, deger) in [
            (bicim::sure(k.sure_ms), k.sure_ms as f64),
            (k.savas.to_string(), k.savas as f64),
            (k.zafer.to_string(), k.zafer as f64),
            (k.wipe.to_string(), k.wipe as f64),
            (
                if k.savas == 0 {
                    "-".into()
                } else {
                    bicim::sayi(k.en_iyi_dps)
                },
                k.en_iyi_dps,
            ),
        ] {
            cikti.push_str(&hucre(&metin, deger, ""));
        }
        cikti.push_str("</tr>");
        let t = bolgeler.entry(Some(k.harita)).or_default();
        t.ad.clone_from(&k.ad);
        t.kosu += 1;
        t.savas += k.savas;
        t.zafer += k.zafer;
        t.sure = t.sure.saturating_add(k.sure_ms);
    }
    cikti.push_str(son(kosular.is_empty()));
    for e in g.savaslar.iter().filter(|e| e.baslik.bolge.is_none()) {
        let t = bolgeler.entry(None).or_default();
        t.ad = "Bölge yok".into();
        t.savas += 1;
        t.zafer += usize::from(e.baslik.sonuc.eq_ignore_ascii_case("Zafer"));
        t.sure = t.sure.saturating_add(e.baslik.sure_ms);
    }
    cikti.push_str(&tablo(
        "Bölgeler",
        &["Bölge", "Koşu", "Savaş", "Zafer", "Toplam süre"],
    ));
    let mut toplamlar: Vec<_> = bolgeler.into_values().collect();
    toplamlar.sort_by_cached_key(|t| t.ad.to_lowercase());
    for t in &toplamlar {
        cikti.push_str(&format!(r#"<tr><td class="sol">{}</td>"#, kacis(&t.ad)));
        for (metin, deger) in [
            (t.kosu.to_string(), t.kosu as f64),
            (t.savas.to_string(), t.savas as f64),
            (t.zafer.to_string(), t.zafer as f64),
            (bicim::sure(t.sure), t.sure as f64),
        ] {
            cikti.push_str(&hucre(&metin, deger, ""));
        }
        cikti.push_str("</tr>");
    }
    cikti.push_str(son(toplamlar.is_empty()));
    cikti
}
