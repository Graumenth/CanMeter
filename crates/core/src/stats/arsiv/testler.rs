//! Arşiv yazıcı: tam dosya gidiş dönüşü, kısa savaş, aynı saniye sırası (hata yolu `testler_kosu.rs`).
use super::{Arsiv, yol::gecici};
use crate::combat::Tablo;
use crate::protocol::mesaj::{Hasar, Kimlik, Mesaj, NpcSpawn, Olum, OyuncuSpawn, Periyodik};
use crate::protocol::mesaj::{Roster, RosterUye};
use crate::stats::gecmis::{Etiket, Neden, Ozet, Sonuc};
use crate::stats::{
    KapananSavas, SavasAnlik, ayrinti::Ayrinti, bolge::Bolge, toplayici::Toplayici,
};
use canmeter_sema::Kayit;
use std::{fs, path::PathBuf};

/// Test başına ayrı geçici klasör (testler paralel koşar); `temizle` siler.
#[rustfmt::skip]
pub(super) fn kok(ad: &str) -> PathBuf {
    std::env::temp_dir().join(format!("canmeter-arsiv-{}", std::process::id())).join(ad)
}
#[rustfmt::skip]
pub(super) fn temizle(k: &PathBuf) {
    let _ = (fs::remove_dir_all(k), fs::remove_file(k), k.parent().map(fs::remove_dir));
}
/// Doğrudan vuruş; aktör hedefe eşitse iyileştirmedir.
#[rustfmt::skip]
pub(super) fn vur(t: &mut Toplayici, z: u64, actor: u32, hedef: u32, deger: u64, tur: u8, beceri_ham: u32) {
    let h = Hasar { actor, hedef, deger, vurus_turu: tur, beceri_ham, anahtar: 0, ek_toplam: 0,
        ek_sayisi: 0, plotter: 0, aci: 0 };
    t.hasar(z, &h, false);
}
/// Yerel 1, partili 2, NPC 90.
#[rustfmt::skip]
pub(super) fn tablo() -> Tablo {
    let mut t = Tablo::new();
    t.isle(&Mesaj::Kimlik(Kimlik { id: 1, ad: "Yerel".into(), sunucu: 1, sinif_kodu: 30,
        seviye: 10 }));
    t.isle(&Mesaj::OyuncuSpawn(OyuncuSpawn { id: 2, ad: "Dost".into(), sinif_kodu: 30 }));
    let uyeler = ["Yerel", "Dost"].map(|s| RosterUye { ad: s.into(), sinif_kodu: 30, seviye: 10 });
    t.isle(&Mesaj::Roster(Roster { uyeler: uyeler.to_vec() }));
    t.isle(&Mesaj::NpcSpawn(NpcSpawn { id: 90, tur: 12, sahip: None, sahip_adi: None, hp: None }));
    t
}
#[rustfmt::skip]
pub(super) fn kapanan(sure_ms: u64, toplam_hasar: u64) -> KapananSavas {
    let ozet = Ozet { kimlik: 1, baslangic_ms: 1_700_000_000_000, sure_ms, neden: Neden::Sessizlik,
        sonuc: Sonuc::Bilinmiyor, etiket: Etiket::Bilinmiyor, hedef: None, toplam_hasar,
        toplam_iyilestirme: 0, katilimci: 0, olum: 0 };
    KapananSavas { ozet, anlik: SavasAnlik::default(), ayrinti: Ayrinti::default(), bolge: None }
}
/// İki kişilik savaş: vuruşlar (kritik, miss), DoT, kendine iyileştirme, NPC'nin öldürdüğü 2.
#[rustfmt::skip]
fn savas(b: &mut Tablo) -> KapananSavas {
    let mut t = Toplayici::new();
    vur(&mut t, 10_000, 1, 90, 1_000, 2, 11); vur(&mut t, 12_000, 2, 90, 700, 3, 22);
    vur(&mut t, 13_000, 1, 90, 500, 1, 11);
    t.periyodik(14_000, &Periyodik { actor: 1, hedef: 90, miktar: 300, beceri_ham: 33,
        etki_turu: 2 });
    vur(&mut t, 15_000, 2, 2, 400, 2, 44); vur(&mut t, 16_000, 90, 2, 900, 2, 55);
    b.isle(&Mesaj::Olum(Olum { id: 2, bayrak: 3 })); t.olum(2);
    vur(&mut t, 20_000, 1, 90, 800, 2, 11);
    t.kapat(Neden::Sessizlik, 60_000, b);
    let a = t.ayrinti();
    let ayrinti = Ayrinti { beceriler: a.beceriler.clone(), kovalar: a.kovalar.clone(),
        yasam: a.yasam.clone(), son_vurus: a.son_vurus.clone(), tasan: a.tasan };
    let bolge = Some(Bolge { harita: 600032, ornek: 1, giris_ms: 1_700_000_000_000 });
    KapananSavas { ozet: t.gecmis()[0].clone(), anlik: t.anlik(60_001, b).unwrap(), ayrinti, bolge }
}
#[test]
#[rustfmt::skip]
fn tam_dosya_yazilir_ve_okunur() {
    let (k, mut b) = (kok("tam"), tablo());
    let yol = Arsiv::new(k.clone()).yaz(&savas(&mut b), &b).unwrap();
    assert!(yol.to_string_lossy().ends_with(".cmenc.jsonl") && !gecici(&yol).exists());
    let metin = fs::read_to_string(&yol).unwrap();
    let satir: Vec<Kayit> = metin
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let (Kayit::Baslik(bas), Some(Kayit::Son(son))) = (&satir[0], satir.last()) else { panic!() };
    let (mut kat, mut bec, mut kova, mut yasam) = (vec![], vec![], vec![], vec![]);
    for s in &satir { match s {
        Kayit::Katilimci(k) => kat.push(k), Kayit::Yasam(y) => yasam.push(y),
        Kayit::Beceri(b) => bec.push((b.katilimci, b.hasar)),
        Kayit::Kova(v) => kova.push((v.katilimci, v.hasar.iter().map(|&x| u64::from(x)).sum())),
        _ => {}
    } }
    assert_eq!((bas.sema, bas.kimlik.len()), (1, 18));
    assert!(bas.baslangic_utc.ends_with('Z') && ["Bilinmiyor", "Zafer"].contains(&&*bas.sonuc));
    let bolge = bas.bolge.as_ref().unwrap();
    assert!(bolge.ad == "Draupnir" && bolge.giris_utc.ends_with('Z'));
    assert_eq!(kat.iter().map(|k| k.no).collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(bas.toplam_hasar, kat.iter().map(|k| k.hasar).sum::<u64>());
    let top = |v: &[(u32, u64)], no| v.iter().filter(|p| p.0 == no).map(|p| p.1).sum::<u64>();
    for k in &kat {
        assert!(kova.iter().any(|p| p.0 == k.no));
        assert_eq!((top(&bec, k.no), top(&kova, k.no)), (k.hasar, k.hasar));
    }
    assert_eq!((yasam.len(), yasam[0].olay.as_str(), yasam[0].katilimci), (1, "olum", 2));
    assert!(yasam[0].kaynak.is_some() && son.tamam && son.kayit as usize == satir.len());
    temizle(&k);
}
#[test]
fn kisa_ya_da_bos_savas_yazilmaz() {
    let (k, b) = (kok("kisa"), tablo());
    let mut a = Arsiv::new(k.clone());
    assert!(a.yaz(&kapanan(2_000, 100), &b).is_none() && a.yaz(&kapanan(10_000, 0), &b).is_none());
    assert!(!k.exists() && a.son_hata.is_none());
}
#[test]
fn ayni_saniyede_iki_dosya() {
    let (k, b) = (kok("sira"), tablo());
    let mut a = Arsiv::new(k.clone());
    let mut yaz = || a.yaz(&kapanan(10_000, 5), &b).unwrap();
    let adlar = [yaz(), yaz()].map(|y| y.to_string_lossy().into_owned());
    assert!(adlar[0].ends_with("-01.cmenc.jsonl") && adlar[1].ends_with("-02.cmenc.jsonl"));
    temizle(&k);
}
