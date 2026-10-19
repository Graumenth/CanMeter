//! Dönüştürücü: diriliş kaydı, nitelik kapsamı, bilinmeyen sınıf, 60'tan uzun kova serisi.
use super::donustur::kayitlar;
use super::testler::{tablo, vur};
use crate::combat::Tablo;
use crate::protocol::mesaj::{Hasar, Kimlik, Mesaj};
use crate::stats::{KapananSavas, ayrinti::Ayrinti, gecmis::Neden, toplayici::Toplayici};
use canmeter_sema::Kayit;

/// Toplayıcıyı kapatıp tam paket kurar (dosya yazmadan).
#[rustfmt::skip]
fn paket(mut t: Toplayici, b: &mut Tablo) -> Vec<Kayit> {
    t.kapat(Neden::Sessizlik, 200_000, b);
    let a = t.ayrinti();
    let ayrinti = Ayrinti { beceriler: a.beceriler.clone(), kovalar: a.kovalar.clone(),
        yasam: a.yasam.clone(), son_vurus: a.son_vurus.clone(), tasan: a.tasan };
    let anlik = t.anlik(200_001, b).unwrap();
    kayitlar(&KapananSavas { ozet: t.gecmis()[0].clone(), anlik, ayrinti }, b, "k", "test")
}

#[test]
fn dirilis_kaydi_yazilir() {
    let (mut t, mut b) = (Toplayici::new(), tablo());
    vur(&mut t, 10_000, 1, 90, 100, 2, 11);
    vur(&mut t, 11_000, 90, 1, 50, 2, 55);
    t.olum(1);
    t.dirildi(1);
    let yasam: Vec<_> = paket(t, &mut b)
        .into_iter()
        .filter_map(|k| {
            if let Kayit::Yasam(y) = k {
                Some(y)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(yasam.len(), 2);
    assert_eq!(
        (yasam[0].olay.as_str(), yasam[1].olay.as_str()),
        ("olum", "dirilis")
    );
    assert!(yasam[1].kaynak.is_none());
}

#[test]
fn kapsam_sifirsa_nitelik_none() {
    let (mut t, mut b) = (Toplayici::new(), tablo());
    for (z, anahtar, beceri_ham) in [(10_000, 0x04, 11), (11_000, 0x06, 22)] {
        let h = Hasar {
            actor: 1,
            hedef: 90,
            deger: 100,
            vurus_turu: 2,
            beceri_ham,
            anahtar,
            ek_toplam: 0,
            ek_sayisi: 0,
            plotter: 0,
            aci: 0,
        };
        t.hasar(z, &h, false);
    }
    for k in paket(t, &mut b) {
        let Kayit::Beceri(s) = k else { continue };
        let n = &s.nitelik;
        match s.kimlik {
            11 => assert_eq!((n.perfect, n.back, n.block), (None, None, None)),
            _ => assert_eq!((n.perfect, n.back, n.block), (Some(0), Some(0), Some(0))),
        }
    }
}

#[test]
fn bilinmeyen_sinif_adi() {
    let (mut t, mut b) = (Toplayici::new(), tablo());
    b.isle(&Mesaj::Kimlik(Kimlik {
        id: 7,
        ad: "Gizem".into(),
        sunucu: 1,
        sinif_kodu: 999,
        seviye: 10,
    }));
    vur(&mut t, 10_000, 7, 90, 100, 2, 11);
    let kat: Vec<_> = paket(t, &mut b)
        .into_iter()
        .filter_map(|k| {
            if let Kayit::Katilimci(c) = k {
                Some(c)
            } else {
                None
            }
        })
        .collect();
    assert_eq!((kat.len(), kat[0].sinif.as_str()), (1, "Bilinmiyor"));
}

#[test]
fn altmis_ustu_kova_parcalanir() {
    let (mut t, mut b) = (Toplayici::new(), tablo());
    for i in 0..70 {
        vur(&mut t, 10_000 + i * 1_000, 1, 90, 10, 2, 11);
    }
    let kovalar: Vec<_> = paket(t, &mut b)
        .into_iter()
        .filter_map(|k| {
            if let Kayit::Kova(v) = k {
                Some(v)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(kovalar.len(), 2);
    assert_eq!((kovalar[0].bas, kovalar[1].bas), (0, 60));
    assert_eq!((kovalar[0].hasar.len(), kovalar[1].hasar.len()), (60, 10));
    let toplam: u32 = kovalar.iter().flat_map(|v| &v.hasar).sum();
    assert_eq!(toplam, 700);
}
