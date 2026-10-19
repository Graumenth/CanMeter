//! Savaş sınırı bağlama testleri: sunucu geçişi, kaynak sonu, sessizlik, kayıt bitişi.
use super::testler_ortak::*;
use super::*;
use crate::stats::gecmis::Sonuc;
use std::{fs, net::Ipv4Addr, time::Duration};

#[test]
fn sunucu_gecisi_acik_savasi_keser() {
    let ((mut boru, mut a), mut b) = (kilitli(), Uc::new(2));
    a.gonder(&mut boru, 1200, &hasar_cercevesi());
    a.gonder(&mut boru, 1300, &hasar_cercevesi());
    for t in (5000..9000).step_by(100) {
        b.gonder(&mut boru, t, &kalp());
    }
    assert_eq!(boru.sunucu(), Some(b.0));
    let g = boru.gecmis();
    assert_eq!(g.len(), 1);
    assert_eq!(
        (g[0].neden, g[0].sonuc),
        (Neden::SunucuDegisti, Sonuc::Kesildi)
    );
    assert_eq!(g[0].toplam_hasar, 2162);
}

#[test]
fn kaynak_sonu_ve_uygulama_kapanisi() {
    let (mut boru, mut a) = kilitli();
    a.gonder(&mut boru, 1200, &hasar_cercevesi());
    boru.kapat(Neden::KaynakSonu, 1300);
    assert_eq!(boru.gecmis().len(), 1);
    assert_eq!(boru.gecmis()[0].neden, Neden::KaynakSonu);
    boru.kapat(Neden::KaynakSonu, 1400);
    assert_eq!(boru.gecmis().len(), 1);
    a.gonder(&mut boru, 1500, &hasar_cercevesi());
    boru.kapat(Neden::UygulamaKapandi, 1600);
    assert_eq!(boru.gecmis().len(), 2);
}

#[test]
fn sessizlik_olaydan_once_kapatir() {
    let (mut boru, mut a) = kilitli();
    a.gonder(&mut boru, 1200, &hasar_cercevesi());
    let ilk = boru.anlik(1300).unwrap().savas_id;
    a.gonder(&mut boru, 40_000, &hasar_cercevesi());
    assert_eq!(boru.gecmis().len(), 1);
    assert_eq!(boru.gecmis()[0].neden, Neden::Sessizlik);
    let anlik = boru.anlik(40_000).unwrap();
    assert!(anlik.aktif);
    assert_eq!(anlik.savas_id, ilk + 1);
}

/// Ethernet/IP/TCP çerçevesi: A ucu 198.51.100.20:13328 -> istemci 192.0.2.1:50000.
fn eth(seq: u32, yuk: &[u8]) -> Vec<u8> {
    let mut v = vec![0; 54];
    v[12..14].copy_from_slice(&[8, 0]);
    v[14] = 0x45;
    v[16..18].copy_from_slice(&(40 + yuk.len() as u16).to_be_bytes());
    v[23] = 6;
    v[26..30].copy_from_slice(&[198, 51, 100, 20]);
    v[30..34].copy_from_slice(&Ipv4Addr::new(192, 0, 2, 1).octets());
    v[34..36].copy_from_slice(&13328u16.to_be_bytes());
    v[36..38].copy_from_slice(&50000u16.to_be_bytes());
    v[38..42].copy_from_slice(&seq.to_be_bytes());
    v[46] = 0x50;
    v.extend_from_slice(yuk);
    v
}

#[test]
fn kayit_kaynagi_bitince_kapatir() {
    let yol = std::env::temp_dir().join(format!("canmeter-savas-{}.pcap", std::process::id()));
    let mut ham = vec![
        0xd4, 0xc3, 0xb2, 0xa1, 2, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0,
    ];
    let mut seq = 1000u32;
    let mut yukler: Vec<Vec<u8>> = (0..12).map(|_| kalp().to_vec()).collect();
    yukler.push(hasar_cercevesi());
    for (n, yuk) in yukler.iter().enumerate() {
        let c = eth(seq, yuk);
        seq = seq.wrapping_add(yuk.len() as u32);
        ham.extend(1u32.to_le_bytes());
        ham.extend((n as u32 * 100_000).to_le_bytes());
        ham.extend((c.len() as u32).to_le_bytes());
        ham.extend((c.len() as u32).to_le_bytes());
        ham.extend(&c);
    }
    fs::write(&yol, ham).unwrap();
    let mut kaynak = crate::kaynak::kayit::KayitKaynak::ac(&yol, true).unwrap();
    let _ = crate::kaynak::AnlikKaynagi::ilerle(&mut kaynak, Duration::ZERO);
    assert!(kaynak.bitti());
    let g = kaynak.boru().gecmis();
    assert_eq!(g.len(), 1);
    assert_eq!(g[0].neden, Neden::KaynakSonu);
    drop(kaynak);
    fs::remove_file(yol).unwrap();
}
