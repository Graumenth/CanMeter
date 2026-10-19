use crate::{hesap, oku::Gecmis};
use chrono::DateTime;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct KosuOzeti {
    pub harita: u32,
    pub ad: String,
    pub giris_utc: String,
    pub sure_ms: u64,
    pub savas: usize,
    pub zafer: usize,
    pub wipe: usize,
    pub en_iyi_dps: f64,
    pub savaslar: Vec<usize>,
}

pub(crate) fn hesapla(g: &Gecmis) -> Vec<KosuOzeti> {
    let mut gruplar = BTreeMap::new();
    for k in &g.kosular {
        gruplar.insert(
            (k.harita, k.giris_utc.as_str()),
            (
                KosuOzeti {
                    harita: k.harita,
                    ad: k.ad.clone(),
                    giris_utc: k.giris_utc.clone(),
                    sure_ms: k.sure_ms,
                    ..Default::default()
                },
                true,
            ),
        );
    }
    for (indeks, e) in g.savaslar.iter().enumerate() {
        let Some(anahtar) = hesap::kosu_anahtari(e) else {
            continue;
        };
        let (k, _) = gruplar.entry(anahtar).or_insert_with(|| {
            (
                KosuOzeti {
                    harita: anahtar.0,
                    giris_utc: anahtar.1.into(),
                    ..Default::default()
                },
                false,
            )
        });
        if k.ad.trim().is_empty()
            && let Some(b) = &e.baslik.bolge
        {
            k.ad.clone_from(&b.ad);
        }
        k.savaslar.push(indeks);
        k.savas += 1;
        k.zafer += usize::from(e.baslik.sonuc.eq_ignore_ascii_case("Zafer"));
        k.wipe += usize::from(e.baslik.sonuc.eq_ignore_ascii_case("OlasiWipe"));
        k.en_iyi_dps = k
            .en_iyi_dps
            .max(hesap::dps(e.baslik.toplam_hasar, e.baslik.sure_ms));
    }
    let mut sonuc: Vec<_> = gruplar
        .into_values()
        .map(|(mut k, kayitli)| {
            k.ad = hesap::bolge_adi(g, k.harita, &k.ad);
            if !kayitli {
                k.sure_ms = sure(g, &k.savaslar);
            }
            k
        })
        .collect();
    sonuc
        .sort_by_cached_key(|k| std::cmp::Reverse(DateTime::parse_from_rfc3339(&k.giris_utc).ok()));
    sonuc
}

fn sure(g: &Gecmis, indeksler: &[usize]) -> u64 {
    let araliklar = indeksler.iter().filter_map(|&i| {
        let b = &g.savaslar[i].baslik;
        let bas = i128::from(
            DateTime::parse_from_rfc3339(&b.baslangic_utc)
                .ok()?
                .timestamp_millis(),
        );
        Some((bas, bas + i128::from(b.sure_ms)))
    });
    let aralik = araliklar.reduce(|(bas, son), (b, s)| (bas.min(b), son.max(s)));
    aralik.map_or(0, |(bas, son)| u64::try_from(son - bas).unwrap_or(u64::MAX))
}

#[cfg(test)]
#[path = "kosu_testler.rs"]
mod testler;
