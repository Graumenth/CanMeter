use super::{bolge, ortak, serit};
use crate::{bicim, hesap, oku::Gecmis};

pub fn yaz(g: &Gecmis) -> String {
    let savaslar = &g.savaslar;
    let mut cikti = ortak::bas("CanMeter geçmiş", "", &serit::yaz(g, None, ""));
    cikti.push_str(
        r#"<div class="baslik"><div><h1>CanMeter geçmiş</h1></div></div><div class="kutular">"#,
    );
    let sure = savaslar
        .iter()
        .fold(0_u64, |s, e| s.saturating_add(e.baslik.sure_ms));
    let zafer = savaslar
        .iter()
        .filter(|e| ortak::sonuc(&e.baslik.sonuc).0 == "zafer")
        .count();
    for (ad, deger) in [
        ("Savaş sayısı", savaslar.len().to_string()),
        ("Toplam süre", bicim::sure(sure)),
        ("Zafer sayısı", zafer.to_string()),
    ] {
        cikti.push_str(&ortak::kutu(ad, &deger));
    }
    cikti.push_str("</div>");
    cikti.push_str(&bolge::yaz(g));
    cikti.push_str(r#"<section class="bolum"><h2>Savaşlar</h2><table>
<thead><tr><th class="sol">Tarih</th><th class="sol">Hedef</th><th>Etiket</th><th>Sonuç</th><th>Süre</th><th>Toplam DPS</th><th>Hasar</th><th>Ölüm</th></tr></thead><tbody>"#);
    for savas in savaslar {
        let b = &savas.baslik;
        let yol = format!("savas/{}.html", ortak::kimlik(&b.kimlik));
        cikti.push_str(&format!(r#"<tr tabindex="0" onclick="location.href='{yol}'" onkeydown="if(event.key==='Enter')location.href='{yol}'">
<td class="sol"><a href="{yol}">{}</a></td><td class="sol">{}</td><td>{}</td><td>{}{}</td>
<td class="sayi" data-v="{}">{}</td><td class="sayi" data-v="{}">{}</td>
<td class="sayi" data-v="{}">{}</td><td class="sayi">{}</td></tr>"#,
            bicim::tarih(&b.baslangic_utc, false), ortak::hedef(savas),
            ortak::rozet(ortak::etiket(&b.etiket)), ortak::rozet(ortak::sonuc(&b.sonuc)), ortak::yarim(savas),
            b.sure_ms, bicim::sure(b.sure_ms), hesap::dps(b.toplam_hasar, b.sure_ms),
            bicim::sayi(hesap::dps(b.toplam_hasar, b.sure_ms)), b.toplam_hasar,
            bicim::sayi(b.toplam_hasar as f64), hesap::olum(savas)));
    }
    cikti.push_str("</tbody></table></section>");
    cikti.push_str(&ortak::son(""));
    cikti
}
