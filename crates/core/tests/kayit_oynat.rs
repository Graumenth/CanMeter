//! Uçtan uca replay: `kayitlar/` altındaki en yeni pcap (git dışı, oyuncu adı içerir) hızlandırılmış oynatılır.
//! Yalnız `CANMETER_KAYIT=1` ile koşar (başka ajanların `cargo test`i kaydı açmasın); dosya yoksa geçer.
//! Çıktıda yalnız sayılar vardır, ad basılmaz.

use std::path::{Path, PathBuf};
use std::time::Duration;

use canmeter_core::Motor;

fn en_yeni_kayit() -> Option<PathBuf> {
    let klasor = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../kayitlar");
    std::fs::read_dir(klasor)
        .ok()?
        .flatten()
        .map(|g| g.path())
        .filter(|p| p.extension().is_some_and(|u| u == "pcap"))
        .max_by_key(|p| p.metadata().and_then(|m| m.modified()).ok())
}

#[test]
fn kayit_uctan_uca_dps_uretir() {
    if std::env::var_os("CANMETER_KAYIT").is_none() {
        eprintln!("CANMETER_KAYIT yok, atlandı");
        return;
    }
    let Some(yol) = en_yeni_kayit() else {
        eprintln!("kayıt yok, atlandı");
        return;
    };
    let mut motor = Motor::kayit(&yol, true).expect("kayıt açılamadı");
    let mut son = None;
    for _ in 0..8 {
        if let Some(anlik) = motor.ilerle(Duration::from_secs(1)) {
            son = Some(anlik);
        }
    }
    let anlik = son.expect("kayıttan hiç anlık üretilmedi");
    eprintln!(
        "savas_id={} sure_ms={} toplam_hasar={} oyuncu={} hedef_var={} adli_oyuncu={}",
        anlik.savas_id,
        anlik.sure_ms,
        anlik.toplam_hasar,
        anlik.oyuncular.len(),
        anlik.hedef.is_some(),
        anlik
            .oyuncular
            .iter()
            .filter(|o| !o.ad.starts_with('#'))
            .count()
    );
    assert!(anlik.toplam_hasar > 0, "kayıtta hasar yok");
    assert!(!anlik.oyuncular.is_empty(), "oyuncu satırı yok");
    assert!(
        anlik.oyuncular.iter().any(|o| o.ben),
        "yerel oyuncu işaretlenmedi"
    );
}
