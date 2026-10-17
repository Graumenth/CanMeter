use super::{grafik, kacis::kacis, ortak, oyuncu, serit};
use crate::{bicim, hesap, sema::Encounter};

pub fn yaz(savaslar: &[Encounter], indeks: usize) -> String {
    let savas = &savaslar[indeks];
    let b = &savas.baslik;
    let baslik = format!(
        "{} · {} · CanMeter",
        ortak::hedef(savas),
        bicim::tarih(&b.baslangic_utc, true)
    );
    let mut cikti = ortak::bas(&baslik, "../", &serit::yaz(savaslar, Some(indeks), "../"));
    cikti.push_str(&format!(
        r#"<div class="baslik"><div><h1>{}</h1>
<div class="meta">{}{}{}{} · süre {} · HP {} · {} oyuncu</div></div>
<div class="meta">CanMeter {} · kayıt {}</div></div>
<div class="kutular">"#,
        ortak::hedef(savas),
        ortak::rozet(ortak::sonuc(&b.sonuc)),
        ortak::rozet(ortak::etiket(&b.etiket)),
        ortak::yarim(savas),
        bicim::tarih(&b.baslangic_utc, false),
        bicim::sure(b.sure_ms),
        bicim::sayi(b.hedef.azami_hp as f64),
        savas.katilimcilar.len(),
        kacis(&b.surum),
        kacis(&b.kimlik)
    ));
    for (ad, deger) in [
        (
            "Toplam DPS",
            bicim::sayi(hesap::dps(b.toplam_hasar, b.sure_ms)),
        ),
        ("Hasar", bicim::sayi(b.toplam_hasar as f64)),
        ("İyileştirme", bicim::sayi(b.toplam_iyilestirme as f64)),
        ("Ölüm", hesap::olum(savas).to_string()),
    ] {
        cikti.push_str(&ortak::kutu(ad, &deger));
    }
    let en = hesap::oyuncular(savas).first().map_or_else(
        || "?".into(),
        |k| {
            format!(
                r#"{} <span class="soluk sayi">{}</span>"#,
                kacis(&k.ad),
                bicim::sayi(hesap::dps(k.hasar, b.sure_ms))
            )
        },
    );
    cikti.push_str(&format!(
        r#"<div class="kutu"><small>En yüksek</small><b>{en}</b></div></div>"#
    ));
    cikti.push_str(&grafik::yaz(savas));
    cikti.push_str(&oyuncu::yaz(savas));
    cikti.push_str(&yasam(savas));
    cikti.push_str(r#"<p class="altbilgi">Vuruş = doğrudan hasar mesajı sayısı; kritik yüzdesi doğrudan vuruşlara göre. "?" alan bu kayıtta çözülmedi. Sınıf ikonları NC Corporation varlığıdır."#);
    for not in &b.kalite {
        cikti.push_str(&format!("<br>Kalite: {}", kacis(not)));
    }
    cikti.push_str("</p>");
    cikti.push_str(&ortak::son("../"));
    cikti
}

fn yasam(savas: &Encounter) -> String {
    let mut cikti =
        String::from(r#"<section class="bolum"><h2>Ölümler</h2><ul class="olum-listesi">"#);
    let mut olaylar: Vec<_> = savas.yasam.iter().collect();
    olaylar.sort_by_key(|y| y.ms);
    for olay in olaylar {
        let eylem = match olay.olay.as_str() {
            "olum" => "öldü",
            "dirilis" => "dirildi",
            _ => continue,
        };
        let ad = savas
            .katilimcilar
            .iter()
            .find(|k| k.no == olay.katilimci)
            .map_or_else(|| format!("#{}", olay.katilimci), |k| kacis(&k.ad));
        let ayrinti: Vec<_> = [olay.kaynak.as_deref(), olay.beceri.as_deref()]
            .into_iter()
            .flatten()
            .filter(|s| !s.is_empty())
            .map(kacis)
            .collect();
        let ayrinti = if ayrinti.is_empty() {
            String::new()
        } else {
            format!("· {}", ayrinti.join(", "))
        };
        cikti.push_str(&format!(r#"<li><span class="zaman sayi">{}</span><span><b>{ad}</b> {eylem} <span class="soluk">{ayrinti}</span></span></li>"#, bicim::sure(olay.ms)));
    }
    cikti.push_str("</ul></section>");
    cikti
}
