//! Kapanan savaştan şema v1 kayıtları. Ad çözümü yalnız burada; dosya sistemine dokunmaz.
use crate::combat::Tablo;
use crate::stats::KapananSavas;
use crate::stats::ayrinti::{Aile, BeceriAnahtar, BeceriSayac, Kova, Olay};
use canmeter_sema as sema;
use chrono::{DateTime, SecondsFormat, Utc};
use std::cmp::Reverse;

/// Kova kaydı başına en çok değer.
const KOVA_PARCA: usize = 60;

/// Kapanan savaşı şema v1 kayıtlarına çevirir (baslik, katilimci*, beceri*, kova*, yasam*, son). Ad çözümü burada.
pub(super) fn kayitlar(
    k: &KapananSavas,
    tablo: &Tablo,
    kimlik: &str,
    surum: &str,
) -> Vec<sema::Kayit> {
    let (a, oy) = (&k.ayrinti, &k.anlik.oyuncular);
    let mut cikti = vec![baslik(k, kimlik, surum)];
    let (mut beceriler, mut kovalar) = (Vec::new(), Vec::new());
    for (i, p) in oy.iter().enumerate() {
        let (no, id) = (i as u32 + 1, p.id as u32);
        let mut kendi: Vec<_> = a.beceriler.iter().filter(|(b, _)| b.kaynak == id).collect();
        kendi.sort_unstable_by_key(|(b, s)| (Reverse(s.hasar), b.beceri, b.aile as u8));
        let (mut vurus, mut kritik) = (0_u32, 0_u32);
        for (b, s) in kendi {
            if b.aile == Aile::Dogrudan {
                vurus = vurus.saturating_add(s.vurus);
                kritik = kritik.saturating_add(s.kritik);
            }
            beceriler.push(beceri(no, b, s));
        }
        if let Some(kv) = a.kovalar.get(&id) {
            kova_parcalari(no, kv, &mut kovalar);
        }
        #[rustfmt::skip]
        cikti.push(sema::Kayit::Katilimci(sema::Katilimci {
            no, ad: p.ad.clone(), ben: p.ben, hasar: p.hasar,
            sinif: Some(p.sinif.ad()).filter(|s| *s != "?").unwrap_or("Bilinmiyor").to_owned(),
            iyilestirme: p.iyilestirme, olum: p.olum, vurus, kritik,
        }));
    }
    cikti.append(&mut beceriler);
    cikti.append(&mut kovalar);
    for y in &a.yasam {
        let Some(i) = oy.iter().position(|p| p.id == u64::from(y.kimlik)) else {
            continue;
        };
        #[rustfmt::skip]
        let kaynak = y.kaynak.map(|id| { let mut ad = String::new(); tablo.ad(id, &mut ad); ad });
        #[rustfmt::skip]
        cikti.push(sema::Kayit::Yasam(sema::Yasam {
            ms: y.ms, katilimci: i as u32 + 1, kaynak,
            olay: match y.olay { Olay::Olum => "olum", Olay::Dirilis => "dirilis" }.to_owned(),
            beceri: y.beceri.map(|id| format!("#{id}")),
        }));
    }
    let kayit = cikti.len() as u32 + 1;
    cikti.push(sema::Kayit::Son(sema::Son { kayit, tamam: true }));
    cikti
}

/// Başlık satırı; kalite bayrakları ayrıntıdan.
fn baslik(k: &KapananSavas, kimlik: &str, surum: &str) -> sema::Kayit {
    let (o, a, oy) = (&k.ozet, &k.ayrinti, &k.anlik.oyuncular);
    let kovasiz = oy.iter().any(|p| !a.kovalar.contains_key(&(p.id as u32)));
    #[rustfmt::skip]
    let kalite = [(a.tasan > 0, "tasan_beceri"), (kovasiz, "kovasiz_kaynak")]
        .iter().filter(|b| b.0).map(|b| b.1.to_owned()).collect();
    #[rustfmt::skip]
    let hedef = o.hedef.as_ref().map_or_else(sema::Hedef::default, |h| sema::Hedef {
        ad: h.ad.clone(), kimlik: h.kimlik, azami_hp: u64::from(h.azami_hp), model: None,
    });
    let utc = DateTime::<Utc>::from_timestamp_millis(o.baslangic_ms as i64);
    let baslangic_utc = utc.map(|z| z.to_rfc3339_opts(SecondsFormat::Secs, true));
    #[rustfmt::skip]
    let b = sema::Baslik {
        sema: sema::SEMA, kimlik: kimlik.to_owned(), sure_ms: o.sure_ms,
        sonuc: format!("{:?}", o.sonuc), etiket: format!("{:?}", o.etiket), hedef,
        toplam_hasar: o.toplam_hasar, toplam_iyilestirme: o.toplam_iyilestirme,
        surum: surum.to_owned(), kalite,
        baslangic_utc: baslangic_utc.unwrap_or_default(),
    };
    sema::Kayit::Baslik(b)
}

/// Beceri satırı; nitelik alanı hiç gelmediyse (`kapsam == 0`) sayılar `None`.
fn beceri(no: u32, b: &BeceriAnahtar, s: &BeceriSayac) -> sema::Kayit {
    let nitelik = |n: u32| (s.kapsam > 0).then_some(n);
    #[rustfmt::skip]
    let aile = match b.aile { Aile::Dogrudan => "dogrudan", Aile::Periyodik => "periyodik" };
    #[rustfmt::skip]
    let satir = sema::Beceri {
        katilimci: no, kimlik: b.beceri, ad: None, aile: aile.to_owned(), vurus: s.vurus,
        tick: s.tick, hasar: s.hasar, iyilestirme: s.iyilestirme, kritik: s.kritik,
        kacinma: s.kacinma, multi_vurus: s.multi_vurus, en_buyuk: s.en_buyuk,
        nitelik: sema::Nitelik {
            perfect: nitelik(s.perfect), smite: None,
            back: nitelik(s.back), block: nitelik(s.block),
        },
    };
    sema::Kayit::Beceri(satir)
}

/// Kovayı 60'lık parçalara böler; kısa vektör sıfırla tamamlanır.
fn kova_parcalari(no: u32, kv: &Kova, cikti: &mut Vec<sema::Kayit>) {
    let uzunluk = kv.hasar.len().max(kv.iyilestirme.len());
    for bas in (0..uzunluk).step_by(KOVA_PARCA) {
        let son = (bas + KOVA_PARCA).min(uzunluk);
        let parca = |v: &[u32]| (bas..son).map(|j| v.get(j).copied().unwrap_or(0)).collect();
        #[rustfmt::skip]
        cikti.push(sema::Kayit::Kova(sema::Kova {
            katilimci: no, bas: bas as u32, genislik_ms: kv.genislik_ms,
            hasar: parca(&kv.hasar), iyilestirme: parca(&kv.iyilestirme),
        }));
    }
}
