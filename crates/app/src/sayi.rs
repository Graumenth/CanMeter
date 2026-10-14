//! Sayı biçimleme: 41.2K, %23.6 / 23.6%, 1:01. React kabuğundaki `sayi.ts` ile birebir.

use crate::dil::Dil;

const BIRIMLER: [&str; 4] = ["", "K", "M", "B"];

/// En çok üç anlamlı basamakla kısaltır (1234 -> 1.23K, 999 -> 999).
pub fn kisalt(n: f64) -> String {
    if !n.is_finite() {
        return n.to_string();
    }
    let isaret = if n < 0.0 { "-" } else { "" };
    let d = n.abs();
    if d < 1000.0 {
        return format!("{isaret}{}", d.round() as u64);
    }
    let mut kademe = ((d.log10() / 3.0).floor() as usize).min(BIRIMLER.len() - 1);
    let mut b = uc_basamak(d / 1000f64.powi(kademe as i32));
    if b >= 1000.0 && kademe < BIRIMLER.len() - 1 {
        kademe += 1;
        b = uc_basamak(d / 1000f64.powi(kademe as i32));
    }
    format!("{isaret}{}{}", ondaliksiz(b), BIRIMLER[kademe])
}

/// JS `toPrecision(3)` karşılığı.
fn uc_basamak(x: f64) -> f64 {
    let carpan = 10f64.powi(2 - x.log10().floor() as i32);
    (x * carpan).round() / carpan
}

/// Gereksiz sıfırları atar: 41.2, 113, 1.5.
fn ondaliksiz(x: f64) -> String {
    let s = format!("{x:.2}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// Oran (0..=1) tek ondalıkla: tr "%23.6", en "23.6%".
pub fn yuzde(p: f32, dil: Dil) -> String {
    let d = format!("{:.1}", p * 100.0);
    match dil {
        Dil::Tr => format!("%{d}"),
        Dil::En => format!("{d}%"),
    }
}

/// Yüzde değeri (0..=100) tam sayıya yuvarlanarak: "%54" / "54%".
pub fn yuzde_tam(y: f32, dil: Dil) -> String {
    let d = y.round() as i64;
    match dil {
        Dil::Tr => format!("%{d}"),
        Dil::En => format!("{d}%"),
    }
}

/// Milisaniye -> "d:ss" ya da "s:dd:ss".
pub fn sure(ms: u64) -> String {
    let t = ms / 1000;
    let (sa, dk, sn) = (t / 3600, (t / 60) % 60, t % 60);
    if sa > 0 {
        format!("{sa}:{dk:02}:{sn:02}")
    } else {
        format!("{dk}:{sn:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kisalt_ornekleri() {
        assert_eq!(kisalt(999.0), "999");
        assert_eq!(kisalt(1234.0), "1.23K");
        assert_eq!(kisalt(41_200.0), "41.2K");
        assert_eq!(kisalt(113_000.0), "113K");
        assert_eq!(kisalt(999_950.0), "1M");
        assert_eq!(kisalt(2_500_000_000.0), "2.5B");
        assert_eq!(kisalt(-1500.0), "-1.5K");
    }

    #[test]
    fn yuzde_ve_sure() {
        assert_eq!(yuzde(0.236, Dil::Tr), "%23.6");
        assert_eq!(yuzde(0.236, Dil::En), "23.6%");
        assert_eq!(yuzde_tam(54.4, Dil::Tr), "%54");
        assert_eq!(sure(61_000), "1:01");
        assert_eq!(sure(3_661_000), "1:01:01");
    }
}
