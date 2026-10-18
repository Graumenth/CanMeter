use super::testler_ortak::*;
use super::*;
use crate::kaynak::Durum;

#[test]
fn tampondaki_hasarlar_kendi_zamanlariyla_islenir() {
    // İlk kilitte iki hasarın zaman farkı korunur; DPS geçiş anına sıkışmaz.
    let (mut boru, mut b) = (Boru::new(), Uc::new(2));
    let govde: Vec<_> =
        "FA A9 01 04 00 81 65 B4 42 AE 00 07 02 5B 0E 12 44 01 00 00 00 9E 55 B9 08 01 00"
            .split_whitespace()
            .map(|bayt| u8::from_str_radix(bayt, 16).unwrap())
            .collect();
    let mut cerceve = Vec::new();
    crate::protocol::varint::yaz((govde.len() + 6) as u32, &mut cerceve);
    cerceve.extend([0x04, 0x38]);
    cerceve.extend(govde);
    let hasar = [&kalp()[..], &cerceve].concat();
    b.ilk(&mut boru, 2000, 12_929, "Deneme");
    for t in (2100..=3100).step_by(100) {
        let yuk: &[u8] = if t == 2100 || t == 3100 {
            &hasar
        } else {
            &kalp()
        };
        b.gonder(&mut boru, t, yuk);
    }
    let anlik = boru.anlik(3100).unwrap();
    assert_eq!((anlik.sure_ms, anlik.toplam_hasar), (1000, 2162));
    assert_eq!(anlik.oyuncular[0].dps, 2162.0);
}

#[test]
fn esikteki_iki_yabanci_uc_tamponu_bosaltmaz() {
    // A sürekli konuşur; ikisi de eşikte olunca daha yeni C bekleyen olur, sonra B onu geri almaz.
    let ((mut boru, mut a), mut b, mut c) = (kilitli(), Uc::new(2), Uc::new(3));
    for t in (2000..6900).step_by(100) {
        a.gonder(&mut boru, t, &kalp());
        if t % 200 == 0 { &mut b } else { &mut c }.gonder(&mut boru, t + 50, &kalp());
    }
    assert_eq!(
        (boru.sunucu(), boru.nesil(), boru.bekleyen_uc()),
        (Some(a.0), 1, Some(c.0))
    );
}

#[test]
fn tavan_sonrasi_taban_imzali_segmentle_baslar() {
    // Tavan aşımından sonra imzasız çöp düşer, sonraki imzalı segmentin kimliği işlenir.
    let ((mut boru, _a), mut b) = (kilitli(), Uc::new(2));
    b.ilk(&mut boru, 5000, 500, "Ilk");
    b.gonder(&mut boru, 5050, &vec![0u8; 600 * 1024]);
    b.gonder(&mut boru, 5060, &[7u8; 100]);
    b.ilk(&mut boru, 5100, 501, "Ikinci");
    let mut t = 5200;
    while boru.sunucu() != Some(b.0) && t < 10_000 {
        b.gonder(&mut boru, t, &kalp());
        t += 100;
    }
    assert_eq!((boru.sunucu(), boru.durum()), (Some(b.0), Durum::Hazir));
}

#[test]
fn tampondaki_bolunmus_cerceve_gecis_sonrasi_birlesir() {
    // Ardışık iki segmente bölünen kimlik çerçevesi geçişte birleşir ve işlenir.
    let ((mut boru, _a), mut b) = (kilitli(), Uc::new(2));
    let cerceve = [&kalp()[..], &kimlik_cercevesi(600, "Bolunmus")].concat();
    let (bir, iki) = cerceve.split_at(cerceve.len() / 2);
    b.gonder(&mut boru, 5000, bir);
    b.gonder(&mut boru, 5010, iki);
    let mut t = 5100;
    while boru.sunucu() != Some(b.0) && t < 10_000 {
        b.gonder(&mut boru, t, &kalp());
        t += 100;
    }
    assert_eq!((boru.sunucu(), boru.durum()), (Some(b.0), Durum::Hazir));
}
