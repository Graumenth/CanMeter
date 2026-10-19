//! Başlık satırı: özet, hedef, kalite bayrakları ve savaşın açıldığı bölge (ad yazım anındaki tablodan).
use super::yol::utc;
use crate::stats::KapananSavas;
use canmeter_sema as sema;

/// Başlık satırı; kalite bayrakları ayrıntıdan, bölge adı tablodan (bilinmiyorsa boş).
pub(super) fn baslik(
    k: &KapananSavas,
    kimlik: &str,
    surum: &str,
    haritalar: &sema::Haritalar,
) -> sema::Kayit {
    let (o, a, oy) = (&k.ozet, &k.ayrinti, &k.anlik.oyuncular);
    let kovasiz = oy.iter().any(|p| !a.kovalar.contains_key(&(p.id as u32)));
    #[rustfmt::skip]
    let kalite = [(a.tasan > 0, "tasan_beceri"), (kovasiz, "kovasiz_kaynak")]
        .iter().filter(|b| b.0).map(|b| b.1.to_owned()).collect();
    #[rustfmt::skip]
    let hedef = o.hedef.as_ref().map_or_else(sema::Hedef::default, |h| sema::Hedef {
        ad: h.ad.clone(), kimlik: h.kimlik, azami_hp: u64::from(h.azami_hp), model: None,
    });
    let bolge = k.bolge.map(|b| sema::Bolge {
        harita: b.harita,
        ad: haritalar.ad(b.harita).unwrap_or_default().to_owned(),
        giris_utc: utc(b.giris_ms),
    });
    #[rustfmt::skip]
    let b = sema::Baslik {
        sema: sema::SEMA, kimlik: kimlik.to_owned(), sure_ms: o.sure_ms,
        sonuc: format!("{:?}", o.sonuc), etiket: format!("{:?}", o.etiket), hedef,
        toplam_hasar: o.toplam_hasar, toplam_iyilestirme: o.toplam_iyilestirme,
        surum: surum.to_owned(), kalite, baslangic_utc: utc(o.baslangic_ms), bolge,
    };
    sema::Kayit::Baslik(b)
}
