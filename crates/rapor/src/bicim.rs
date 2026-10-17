use chrono::{DateTime, Datelike, Local, Timelike};

pub fn sayi(deger: f64) -> String {
    let (bolen, ek) = match deger.abs() {
        x if x >= 1_000_000_000.0 => (1_000_000_000.0, "B"),
        x if x >= 1_000_000.0 => (1_000_000.0, "M"),
        x if x >= 1_000.0 => (1_000.0, "K"),
        _ => return format!("{deger:.0}"),
    };
    format!("{:.1}{ek}", deger / bolen).replace('.', ",")
}

pub fn sure(ms: u64) -> String {
    let sn = ms / 1_000;
    if sn >= 3_600 {
        format!("{}:{:02}:{:02}", sn / 3_600, sn / 60 % 60, sn % 60)
    } else {
        format!("{}:{:02}", sn / 60, sn % 60)
    }
}

pub fn yuzde(oran: f64) -> String {
    format!("{:.1}%", oran * 100.0).replace('.', ",")
}

pub fn tarih(metin: &str, kisa: bool) -> String {
    let Ok(utc) = DateTime::parse_from_rfc3339(metin) else {
        return "Bilinmeyen tarih".into();
    };
    let yerel = utc.with_timezone(&Local);
    let aylar = [
        "Ocak", "Şubat", "Mart", "Nisan", "Mayıs", "Haziran", "Temmuz", "Ağustos", "Eylül", "Ekim",
        "Kasım", "Aralık",
    ];
    let ay = aylar[yerel.month0() as usize];
    if kisa {
        let ay: String = ay.chars().take(3).collect();
        format!(
            "{} {ay} {:02}:{:02}",
            yerel.day(),
            yerel.hour(),
            yerel.minute()
        )
    } else {
        format!(
            "{} {ay} {}, {:02}:{:02}",
            yerel.day(),
            yerel.year(),
            yerel.hour(),
            yerel.minute()
        )
    }
}
