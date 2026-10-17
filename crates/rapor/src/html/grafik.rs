use super::kacis::kacis;
use crate::{bicim, hesap, sema::Encounter};

pub fn yaz(savas: &Encounter) -> String {
    let oyuncular = hesap::oyuncular(savas);
    let cizgiler: Vec<_> = oyuncular
        .iter()
        .take(12)
        .map(|k| hesap::ortalamalar(savas, k.no))
        .collect();
    let azami = cizgiler.iter().flatten().copied().fold(0.0_f64, f64::max);
    let tavan = (azami / 10_000.0).ceil().max(1.0) * 10_000.0;
    let sure = savas.baslik.sure_ms as f64 / 1_000.0;
    let mut cikti = format!(
        r#"<section class="bolum grafik"><h2>DPS zaman çizgisi</h2>
<svg viewBox="0 0 1000 230" preserveAspectRatio="none">
<line class="eksen" x1="40" y1="200" x2="990" y2="200"/>
<line class="eksen" x1="40" y1="10" x2="40" y2="200"/>
<text class="etiket" x="36" y="14" text-anchor="end">{}</text>
<text class="etiket" x="36" y="110" text-anchor="end">{}</text>
<text class="etiket" x="36" y="204" text-anchor="end">0</text>
<text class="etiket" x="40" y="222">0:00</text>
<text class="etiket" x="515" y="222" text-anchor="middle">{}</text>
<text class="etiket" x="990" y="222" text-anchor="end">{}</text>
"#,
        bicim::sayi(tavan),
        bicim::sayi(tavan / 2.0),
        bicim::sure(savas.baslik.sure_ms / 2),
        bicim::sure(savas.baslik.sure_ms)
    );
    for (oyuncu, degerler) in oyuncular.iter().zip(&cizgiler) {
        let mut noktalar = String::from("40,200");
        for (indeks, deger) in degerler.iter().enumerate() {
            let sn = (indeks + 1) as f64;
            let x = 40.0 + 950.0 * hesap::oran(sn.min(sure), sure);
            let y = 200.0 - 190.0 * deger / tavan;
            noktalar.push_str(&format!(" {x:.2},{y:.2}"));
        }
        cikti.push_str(&format!(
            r#"<polyline class="cizgi" stroke="{}" points="{noktalar}"/>"#,
            hesap::renk(&oyuncu.sinif)
        ));
    }
    for olay in savas.yasam.iter().filter(|y| y.olay == "olum") {
        let x = 40.0 + 950.0 * hesap::oran(olay.ms as f64, savas.baslik.sure_ms as f64).min(1.0);
        let ad = savas
            .katilimcilar
            .iter()
            .find(|k| k.no == olay.katilimci)
            .map_or_else(|| format!("#{}", olay.katilimci), |k| kacis(&k.ad));
        cikti.push_str(&format!(
            r##"<line class="olum" x1="{x:.2}" y1="10" x2="{x:.2}" y2="200"/>
<text class="etiket" x="{:.2}" y="22" fill="#e5484d">† {ad}</text>"##,
            x + 4.0
        ));
    }
    cikti.push_str("</svg><div class=\"gosterge\">");
    for k in oyuncular {
        cikti.push_str(&format!(
            r#"<span><i style="background:{}"></i>{}</span>"#,
            hesap::renk(&k.sinif),
            kacis(&k.ad)
        ));
    }
    cikti.push_str("</div></section>");
    cikti
}
