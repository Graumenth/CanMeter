use super::*;
use crate::protocol::varint;

fn mesaj(tur: [u8; 2], govde: &[u8]) -> Vec<u8> {
    let mut b = Vec::new();
    varint::yaz((govde.len() + 6) as u32, &mut b);
    b.extend(tur);
    b.extend(govde);
    b
}

fn bundle(ic: &[u8]) -> Vec<u8> {
    let k = lz4_flex::block::compress(ic);
    let mut yuk = vec![0xff, 0xff];
    yuk.extend((ic.len() as u32).to_le_bytes());
    yuk.extend(k);
    cerceve(&yuk)
}

fn cerceve(yuk: &[u8]) -> Vec<u8> {
    let mut b = Vec::new();
    varint::yaz((yuk.len() + 4) as u32, &mut b);
    b.extend(yuk);
    b
}

fn turleri_yurut(b: &[u8]) -> (usize, Vec<[u8; 2]>) {
    let mut y = Yuruyucu::new();
    let mut t = Vec::new();
    let n = y.yurut(b, &mut |tur, _| t.push(tur));
    (n, t)
}

#[test]
fn iki_mesaj() {
    let mut b = mesaj([1, 2], b"a");
    b.extend(mesaj([3, 4], b"bc"));
    assert_eq!(turleri_yurut(&b), (b.len(), vec![[1, 2], [3, 4]]));
}

#[test]
fn mesaj_bundle_mesaj() {
    let mut ic = mesaj([5, 6], b"x");
    ic.extend(mesaj([7, 8], b"y"));
    let mut b = mesaj([1, 2], b"");
    b.extend(bundle(&ic));
    b.extend(mesaj([3, 4], b""));
    assert_eq!(turleri_yurut(&b).1, vec![[1, 2], [5, 6], [7, 8], [3, 4]]);
}

#[test]
fn iki_derinlik() {
    assert_eq!(
        turleri_yurut(&bundle(&bundle(&mesaj([9, 10], b"ic")))).1,
        vec![[9, 10]]
    );
}

#[test]
fn bozuk_bundle_atlanir() {
    let mut b = cerceve(&[0xff, 0xff, 8, 0, 0, 0, 1, 2, 3]);
    b.extend(mesaj([4, 2], b"ok"));
    assert_eq!(turleri_yurut(&b), (b.len(), vec![[4, 2]]));
}

#[test]
fn parcalarda_eksik_son_kalir() {
    let mut kaynak = mesaj([1, 1], b"");
    kaynak.extend(mesaj([2, 2], &vec![8; 200]));
    kaynak.extend(mesaj([3, 3], &vec![9; 2000]));
    let beklenen = vec![
        ([1, 1], vec![]),
        ([2, 2], vec![8; 200]),
        ([3, 3], vec![9; 2000]),
    ];
    for parca in [1, 7, 100, 1460] {
        let mut y = Yuruyucu::new();
        let (mut bekleyen, mut sonuc) = (Vec::new(), Vec::new());
        for p in kaynak.chunks(parca) {
            bekleyen.extend(p);
            let n = y.yurut(&bekleyen, &mut |t, g| sonuc.push((t, g.to_vec())));
            bekleyen.drain(..n);
        }
        assert!(bekleyen.is_empty());
        assert_eq!(sonuc, beklenen);
    }
}
