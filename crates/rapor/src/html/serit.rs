use super::ortak::{self, etiket, sonuc};
use crate::{bicim, hesap, sema::Encounter};

pub fn dosya_kimligi(metin: &str) -> String {
    ortak::kimlik(metin)
}

pub fn yaz(savaslar: &[Encounter], aktif: Option<usize>, onek: &str) -> String {
    let mut cikti = format!(
        r#"<aside class="serit">
<div class="marka"><b>CanMeter</b><span>geçmiş · {} savaş</span></div>
<div class="cipler">
"#,
        savaslar.len()
    );
    for (grup, deger, ad) in [
        ("etiket", "hepsi", "Hepsi"),
        ("etiket", "boss", "Boss"),
        ("etiket", "trash", "Trash"),
        ("etiket", "pvp", "PvP"),
        ("etiket", "bilinmiyor", "Bilinmiyor"),
        ("sonuc", "hepsi", "Her sonuç"),
        ("sonuc", "zafer", "Zafer"),
        ("sonuc", "olasiwipe", "Wipe"),
        ("sonuc", "bilinmiyor", "Bilinmiyor"),
        ("sonuc", "kesildi", "Kesildi"),
    ] {
        let secili = if deger == "hepsi" { " secili" } else { "" };
        cikti.push_str(&format!(
            r#"<span class="cip{secili}" data-grup="{grup}" data-deger="{deger}">{ad}</span>"#
        ));
    }
    cikti.push_str("</div><nav class=\"liste\">");
    for (indeks, savas) in savaslar.iter().enumerate() {
        let b = &savas.baslik;
        let acik = if aktif == Some(indeks) { " acik" } else { "" };
        let (etiket, ad) = etiket(&b.etiket);
        let (sonuc, _) = sonuc(&b.sonuc);
        cikti.push_str(&format!(r#"<a class="kart{acik}" href="{onek}savas/{}.html" data-etiket="{etiket}" data-sonuc="{sonuc}">
<span class="hedef">{}{}{}</span>
<span class="dps sayi">{}<small>DPS</small></span>
<span class="alt">{} · {} · {ad}</span>
</a>"#, ortak::kimlik(&b.kimlik), ortak::rozet(ortak::sonuc(&b.sonuc)),
            ortak::hedef(savas), ortak::yarim(savas), bicim::sayi(hesap::dps(b.toplam_hasar, b.sure_ms)),
            bicim::tarih(&b.baslangic_utc, true), bicim::sure(b.sure_ms)));
    }
    cikti.push_str("</nav></aside>");
    cikti
}
