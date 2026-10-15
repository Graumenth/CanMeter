use std::{
    fs,
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};

use super::*;
use crate::kaynak::{
    AnlikKaynagi,
    canli::{CanliKaynak, HamSegment},
};

fn segment(uc: SocketAddrV4, seq: u32, yuk: &[u8]) -> Segment<'_> {
    Segment {
        kaynak: uc,
        hedef: SocketAddrV4::new(Ipv4Addr::new(192, 0, 2, 1), 50000),
        seq,
        yuk,
    }
}

fn cerceve(uc: SocketAddrV4, yuk: &[u8]) -> Vec<u8> {
    let mut veri = vec![0; 54];
    veri[12..14].copy_from_slice(&[8, 0]);
    veri[14] = 0x45;
    veri[16..18].copy_from_slice(&(40 + yuk.len() as u16).to_be_bytes());
    veri[23] = 6;
    veri[26..30].copy_from_slice(&uc.ip().octets());
    veri[30..34].copy_from_slice(&[192, 0, 2, 1]);
    veri[34..36].copy_from_slice(&uc.port().to_be_bytes());
    veri[36..38].copy_from_slice(&50000u16.to_be_bytes());
    veri[46] = 0x50;
    veri.extend_from_slice(yuk);
    veri
}

#[test]
fn on_iki_imza_sunucuyu_kilitler() {
    let uc = SocketAddrV4::new(Ipv4Addr::new(198, 51, 100, 9), 9000);
    let mut boru = Boru::new();
    for n in 0..IMZA_ESIGI {
        boru.segment(u64::from(n) * 100, &segment(uc, n, &[0x0e, 0, 0x36]));
    }
    assert_eq!(boru.sunucu(), Some(uc));
}

#[test]
fn on_bir_imza_kilitlemez() {
    let uc = SocketAddrV4::new(Ipv4Addr::new(198, 51, 100, 10), 9001);
    let mut boru = Boru::new();
    for n in 0..IMZA_ESIGI - 1 {
        boru.segment(u64::from(n) * 100, &segment(uc, n, &[0x06, 0, 0x36]));
    }
    assert_eq!(boru.sunucu(), None);
}

#[test]
fn canli_kanali_ana_is_parcaciginda_tuketilir() {
    let uc = SocketAddrV4::new(Ipv4Addr::new(198, 51, 100, 11), 9002);
    let (gonderici, alici) = CanliKaynak::gonderici_olustur();
    gonderici
        .send(HamSegment {
            zaman_ms: 42,
            veri: cerceve(uc, &[0x0e, 0, 0x36]),
        })
        .unwrap();
    let mut kaynak = CanliKaynak::kanaldan(alici);
    assert_eq!(kaynak.ilerle(Duration::ZERO), None);
}

#[test]
fn hizli_kayit_dosya_sonuna_ilerler() {
    let kok = std::env::temp_dir().join(format!("canmeter-kayit-{}", std::process::id()));
    let girdi = kok.with_extension("girdi");
    let cikti = kok.with_extension("pcap");
    let cerceve = cerceve(
        SocketAddrV4::new(Ipv4Addr::new(198, 51, 100, 12), 9003),
        &[1],
    );
    let mut ham = vec![
        0xd4, 0xc3, 0xb2, 0xa1, 2, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0,
    ];
    ham.extend(1u32.to_le_bytes());
    ham.extend(0u32.to_le_bytes());
    ham.extend((cerceve.len() as u32).to_le_bytes());
    ham.extend((cerceve.len() as u32).to_le_bytes());
    ham.extend(&cerceve);
    fs::write(&girdi, ham).unwrap();
    let mut kaynak = pcap::Capture::from_file(&girdi).unwrap();
    let olu = pcap::Capture::dead(pcap::Linktype::ETHERNET).unwrap();
    let mut yazici = olu.savefile(&cikti).unwrap();
    yazici.write(&kaynak.next_packet().unwrap());
    yazici.flush().unwrap();
    drop(yazici);
    drop(kaynak);
    let mut kayit = crate::kaynak::kayit::KayitKaynak::ac(&cikti, true).unwrap();
    let _ = kayit.ilerle(Duration::ZERO);
    assert!(kayit.bitti());
    drop(kayit);
    fs::remove_file(girdi).unwrap();
    fs::remove_file(cikti).unwrap();
}
