pub use crate::kosu::KosuOzeti;
use crate::oku::Gecmis;
use crate::sema::{Beceri, Encounter, Katilimci};

pub fn kosular(g: &Gecmis) -> &[KosuOzeti] {
    g.kosu_ozetleri.get_or_init(|| crate::kosu::hesapla(g))
}

pub fn bolge_adi(g: &Gecmis, harita: u32, dosyadaki: &str) -> String {
    g.haritalar.ad(harita).map_or_else(
        || {
            if dosyadaki.trim().is_empty() {
                format!("Bölge #{harita}")
            } else {
                dosyadaki.into()
            }
        },
        str::to_string,
    )
}

pub fn kosu_anahtari(e: &Encounter) -> Option<(u32, &str)> {
    e.baslik
        .bolge
        .as_ref()
        .map(|b| (b.harita, b.giris_utc.as_str()))
}

pub fn oran(pay: f64, toplam: f64) -> f64 {
    pay / toplam.max(1.0)
}

pub fn dps(miktar: u64, sure_ms: u64) -> f64 {
    oran(miktar as f64, sure_ms as f64 / 1_000.0)
}

pub fn oyuncular(savas: &Encounter) -> Vec<&Katilimci> {
    let mut sonuc: Vec<_> = savas.katilimcilar.iter().collect();
    sonuc.sort_by_key(|k| std::cmp::Reverse(k.hasar));
    sonuc
}

pub fn iyilestirme(beceri: &Beceri) -> bool {
    beceri.hasar == 0 && beceri.iyilestirme > 0
}

pub fn miktar(beceri: &Beceri) -> u64 {
    if iyilestirme(beceri) {
        beceri.iyilestirme
    } else {
        beceri.hasar
    }
}

pub fn beceriler(savas: &Encounter, no: u32) -> Vec<&Beceri> {
    let mut sonuc: Vec<_> = savas
        .beceriler
        .iter()
        .filter(|b| b.katilimci == no)
        .collect();
    sonuc.sort_by_key(|b| std::cmp::Reverse(miktar(b)));
    sonuc
}

pub fn olum(savas: &Encounter) -> u64 {
    savas.katilimcilar.iter().map(|k| u64::from(k.olum)).sum()
}

pub fn renk(sinif: &str) -> &'static str {
    match sinif.to_ascii_lowercase().as_str() {
        "gladiator" => "#e05a4e",
        "templar" => "#7fa8f5",
        "assassin" => "#a3d65a",
        "ranger" => "#5fcf78",
        "sorcerer" => "#b08cf0",
        "spiritmaster" => "#c76bd9",
        "cleric" => "#f3d660",
        "chanter" => "#e07d2c",
        _ => "#8a8f98",
    }
}

pub fn ortalamalar(savas: &Encounter, no: u32) -> Vec<f64> {
    let uzunluk = savas.baslik.sure_ms.div_ceil(1_000).max(1) as usize;
    let mut kovalar = vec![0.0; uzunluk];
    for kova in savas.kovalar.iter().filter(|k| k.katilimci == no) {
        let n = (kova.genislik_ms / 1_000).max(1);
        for (indeks, &hasar) in kova.hasar.iter().enumerate() {
            let bas = (kova.bas as usize + indeks) * n as usize;
            // Kalan ilk saniyeye eklenir; kova toplamı korunur.
            for (saniye, hucre) in kovalar.iter_mut().skip(bas).take(n as usize).enumerate() {
                *hucre += f64::from(hasar / n + if saniye == 0 { hasar % n } else { 0 });
            }
        }
    }
    // Pencere mevcut saniye ve önceki dokuz saniyeyi kapsar (10 sn kayan ortalama); başta mevcut kovalar kullanılır.
    (0..uzunluk)
        .map(|i| {
            let bas = i.saturating_sub(9);
            kovalar[bas..=i].iter().sum::<f64>() / (i - bas + 1) as f64
        })
        .collect()
}
