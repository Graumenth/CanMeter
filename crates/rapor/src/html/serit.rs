use super::ortak::{self, etiket, sonuc};
use crate::{bicim, hesap, sema::Encounter};

/// Filtre satiri: veri grubu, gorunen baslik, (deger, ad) ciftleri.
type Grup = (
    &'static str,
    &'static str,
    &'static [(&'static str, &'static str)],
);

pub fn dosya_kimligi(metin: &str) -> String {
    ortak::kimlik(metin)
}

pub fn yaz(savaslar: &[Encounter], aktif: Option<usize>, onek: &str) -> String {
    let mut cikti = format!(
        r#"<aside class="serit">
<div class="marka"><b>CanMeter</b><span>geçmiş · {} savaş</span></div>
"#,
        savaslar.len()
    );
    // Iki ayri satir: tur ve sonuc; her satir kendi icinde kirilir, gruplar karismaz.
    let gruplar: [Grup; 2] = [
        (
            "etiket",
            "Tür",
            &[
                ("hepsi", "Hepsi"),
                ("boss", "Boss"),
                ("trash", "Trash"),
                ("pvp", "PvP"),
            ],
        ),
        (
            "sonuc",
            "Sonuç",
            &[
                ("hepsi", "Hepsi"),
                ("zafer", "Zafer"),
                ("olasiwipe", "Wipe"),
            ],
        ),
    ];
    for (grup, baslik, cipler) in gruplar {
        cikti.push_str(&format!(r#"<div class="cipler"><small>{baslik}</small>"#));
        for (deger, ad) in cipler {
            let secili = if *deger == "hepsi" { " secili" } else { "" };
            cikti.push_str(&format!(
                r#"<span class="cip{secili}" data-grup="{grup}" data-deger="{deger}">{ad}</span>"#
            ));
        }
        cikti.push_str(
            "</div>
",
        );
    }
    cikti.push_str("<nav class=\"liste\">");
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
