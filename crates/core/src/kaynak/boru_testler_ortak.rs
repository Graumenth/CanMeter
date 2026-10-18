//! Boru testleri için ortak yardımcılar: segment, kalp çerçevesi, kimlik çerçevesi, ardışık uç.
use super::*;
use crate::protocol::{mesaj::Mesaj, varint::yaz};

pub(super) fn segment(uc: SocketAddrV4, seq: u32, yuk: &[u8]) -> Segment<'_> {
    Segment {
        kaynak: uc,
        hedef: SocketAddrV4::new([192, 0, 2, 1].into(), 50000),
        seq,
        yuk,
    }
}

pub(super) fn kalp() -> [u8; 11] {
    [0x0e, 0x00, 0x36, 0, 0, 0, 0, 0, 0, 0, 0]
}

pub(super) fn kimlik_cercevesi(id: u32, ad: &str) -> Vec<u8> {
    // Düzen kimlik_testler.rs ile aynı: id, 4 sıfır, maske, ad, sunucu, sınıf, 1 bayt, seviye.
    let mut g = Vec::new();
    yaz(id, &mut g);
    g.extend([0, 0, 0, 0, 0b101, u8::try_from(ad.len()).unwrap()]);
    g.extend(ad.bytes().chain(1304_u16.to_le_bytes()));
    g.extend(30_u32.to_le_bytes().into_iter().chain([1]));
    g.extend(28_u32.to_le_bytes());
    assert!(matches!(coz([0x33, 0x36], &g), Some(Mesaj::Kimlik(_))));
    let mut c = Vec::new();
    yaz(u32::try_from(6 + g.len()).unwrap(), &mut c);
    [c, vec![0x33, 0x36], g].concat()
}

/// Tek uçtan ardışık sıra numaralı gönderici.
pub(super) struct Uc(pub(super) SocketAddrV4, pub(super) u32);
impl Uc {
    pub(super) fn new(son: u8) -> Self {
        Self(SocketAddrV4::new([198, 51, 100, son].into(), 9000), 1000)
    }
    pub(super) fn gonder(&mut self, boru: &mut Boru, t: u64, yuk: &[u8]) {
        boru.segment(t, &segment(self.0, self.1, yuk));
        self.1 = self.1.wrapping_add(yuk.len() as u32);
    }
    pub(super) fn ilk(&mut self, boru: &mut Boru, t: u64, id: u32, ad: &str) {
        self.gonder(boru, t, &[&kalp()[..], &kimlik_cercevesi(id, ad)].concat());
    }
}

/// A ucunu t = 0..1100 arasında 12 kalple kilitler.
pub(super) fn kilitli() -> (Boru, Uc) {
    let (mut boru, mut a) = (Boru::new(), Uc::new(1));
    for n in 0..12 {
        a.gonder(&mut boru, n * 100, &kalp());
    }
    assert_eq!((boru.sunucu(), boru.nesil()), (Some(a.0), 1));
    (boru, a)
}
