use super::kacis::kacis;
use crate::sema::Encounter;

pub fn kimlik(metin: &str) -> String {
    if !metin.is_empty()
        && metin
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return metin.into();
    }
    let mut sonuc = String::from("hex-");
    for bayt in metin.bytes() {
        sonuc.push_str(&format!("{bayt:02x}"));
    }
    sonuc
}

pub fn hedef(savas: &Encounter) -> String {
    if savas.baslik.hedef.ad.is_empty() {
        format!("#{}", savas.baslik.hedef.kimlik)
    } else {
        kacis(&savas.baslik.hedef.ad)
    }
}

pub fn sonuc(metin: &str) -> (&'static str, &'static str) {
    match metin.to_ascii_lowercase().as_str() {
        "zafer" => ("zafer", "Zafer"),
        "olasiwipe" => ("olasiwipe", "Wipe"),
        "kesildi" => ("kesildi", "Kesildi"),
        _ => ("bilinmiyor", "?"),
    }
}

pub fn etiket(metin: &str) -> (&'static str, &'static str) {
    match metin.to_ascii_lowercase().as_str() {
        "boss" => ("boss", "Boss"),
        "trash" => ("trash", "Trash"),
        "pvp" => ("pvp", "PvP"),
        _ => ("bilinmiyor", "Bilinmiyor"),
    }
}

pub fn rozet((sinif, ad): (&str, &str)) -> String {
    format!(r#"<span class="rozet {sinif}">{ad}</span>"#)
}

pub fn yarim(savas: &Encounter) -> &'static str {
    if savas.tamam {
        ""
    } else {
        r#"<span class="rozet kesildi">yarım kayıt</span>"#
    }
}

pub fn bas(baslik: &str, onek: &str, serit: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="tr">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{baslik}</title>
<link rel="stylesheet" href="{onek}stil.css">
</head>
<body>
<div class="duzen">
{serit}
<main class="ana">
"#
    )
}

pub fn son(onek: &str) -> String {
    format!(
        r#"</main>
</div>
<script src="{onek}betik.js"></script>
</body>
</html>
"#
    )
}

pub fn kutu(ad: &str, deger: &str) -> String {
    format!(r#"<div class="kutu"><small>{ad}</small><b class="sayi">{deger}</b></div>"#)
}

pub fn hucre(deger: &str, sayi: f64, ek: &str) -> String {
    let yok = if sayi == 0.0 { " yok" } else { "" };
    format!(r#"<td class="sayi{yok}{ek}" data-v="{sayi}">{deger}</td>"#)
}
