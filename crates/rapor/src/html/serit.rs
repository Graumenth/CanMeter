use super::ortak::{self, etiket, sonuc};
use super::serit_kosu;
use crate::{bicim, hesap, oku::Gecmis, sema::Encounter};

/// Filtre satiri: veri grubu, gorunen baslik, (deger, ad) ciftleri.
type Grup = (
    &'static str,
    &'static str,
    &'static [(&'static str, &'static str)],
);

pub fn dosya_kimligi(metin: &str) -> String {
    ortak::kimlik(metin)
}

pub fn yaz(g: &Gecmis, aktif: Option<usize>, onek: &str) -> String {
    let mut cikti = format!(
        r#"<aside class="serit">
<div class="marka"><b>CanMeter</b><span>geçmiş · {} savaş</span></div>
"#,
        g.savaslar.len()
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
    let kosular = hesap::kosular(g);
    cikti.push_str(&serit_kosu::secim(g, kosular));
    cikti.push_str("<nav class=\"liste\">");
    for k in kosular.iter().filter(|k| k.savas > 0) {
        cikti.push_str(&serit_kosu::baslik(
            Some(k.harita),
            &k.ad,
            Some(&k.giris_utc),
            k.savas,
        ));
        for &indeks in &k.savaslar {
            cikti.push_str(&kart(&g.savaslar[indeks], aktif == Some(indeks), onek));
        }
    }
    let bolgesiz: Vec<_> = g
        .savaslar
        .iter()
        .enumerate()
        .filter(|(_, e)| e.baslik.bolge.is_none())
        .collect();
    if !bolgesiz.is_empty() {
        cikti.push_str(&serit_kosu::baslik(None, "Bölge yok", None, bolgesiz.len()));
        for (indeks, savas) in bolgesiz {
            cikti.push_str(&kart(savas, aktif == Some(indeks), onek));
        }
    }
    cikti.push_str("</nav></aside>");
    cikti
}

fn kart(savas: &Encounter, aktif: bool, onek: &str) -> String {
    let b = &savas.baslik;
    let acik = if aktif { " acik" } else { "" };
    let bolge = b
        .bolge
        .as_ref()
        .map_or_else(|| "yok".into(), |b| b.harita.to_string());
    let (etiket, ad) = etiket(&b.etiket);
    let (sonuc, _) = sonuc(&b.sonuc);
    format!(
        r#"<a class="kart{acik}" href="{onek}savas/{}.html" data-bolge="{bolge}" data-etiket="{etiket}" data-sonuc="{sonuc}">
<span class="hedef">{}{}{}</span>
<span class="dps sayi">{}<small>DPS</small></span>
<span class="alt">{} · {} · {ad}</span>
</a>"#,
        ortak::kimlik(&b.kimlik),
        ortak::rozet(ortak::sonuc(&b.sonuc)),
        ortak::hedef(savas),
        ortak::yarim(savas),
        bicim::sayi(hesap::dps(b.toplam_hasar, b.sure_ms)),
        bicim::tarih(&b.baslangic_utc, true),
        bicim::sure(b.sure_ms)
    )
}
