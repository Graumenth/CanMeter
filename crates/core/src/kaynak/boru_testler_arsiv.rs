//! Arşiv bağlama: kapanan paket yalnız arşiv açıkken ve bir kez tüketilir.
use super::testler_ortak::*;
use super::*;

#[test]
fn arsiv_kapaliyken_yazmaz_acikken_bir_kez() {
    let (mut kapali, mut a) = kilitli();
    a.gonder(&mut kapali, 1200, &hasar_cercevesi());
    kapali.kapat(Neden::KaynakSonu, 1300);
    assert!(kapali.arsiv_hatasi().is_none());
    // Arşiv kapalıyken paket dokunulmadan bekler.
    assert!(kapali.toplayici.yazilacak().is_some());

    let (mut boru, mut b) = kilitli();
    let kok = std::env::temp_dir().join(format!("canmeter-boru-arsiv-{}", std::process::id()));
    boru.arsiv_ayarla(Some(kok));
    b.gonder(&mut boru, 1200, &hasar_cercevesi());
    boru.kapat(Neden::KaynakSonu, 1300);
    assert!(boru.toplayici.yazilacak().is_none());
    boru.anlik(1400);
    assert!(boru.toplayici.yazilacak().is_none());
    boru.arsiv_ayarla(None);
    assert!(boru.arsiv_hatasi().is_none());
}
