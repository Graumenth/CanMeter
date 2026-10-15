use super::*;

#[test]
fn sirali() {
    let mut akis = Akis::new();
    akis.ekle(1, b"ab");
    akis.ekle(3, b"cd");
    assert_eq!(akis.veri(), b"abcd");
}
#[test]
fn tekrar() {
    let mut akis = Akis::new();
    akis.ekle(1, b"ab");
    akis.ekle(1, b"ab");
    assert_eq!(akis.veri(), b"ab");
}
#[test]
fn cakisma() {
    let mut akis = Akis::new();
    akis.ekle(10, b"abc");
    akis.ekle(12, b"cde");
    assert_eq!(akis.veri(), b"abcde");
}
#[test]
fn sira_disi() {
    let mut akis = Akis::new();
    akis.ekle(1, b"a");
    akis.ekle(3, b"c");
    akis.ekle(2, b"b");
    assert_eq!(akis.veri(), b"abc");
}
#[test]
fn sarmal() {
    let mut akis = Akis::new();
    akis.ekle(u32::MAX - 2, b"abc");
    akis.ekle(0, b"d");
    assert_eq!(akis.veri(), b"abcd");
}
#[test]
fn tavan() {
    let mut akis = Akis::new();
    akis.ekle(1, b"a");
    akis.ekle(3, &vec![0; BEKLEYEN_TAVANI + 1]);
    assert_eq!(akis.veri().len(), BEKLEYEN_TAVANI + 2);
}
#[test]
fn tuket() {
    let mut akis = Akis::new();
    akis.ekle(1, b"abc");
    akis.tuket(2);
    assert_eq!(akis.veri(), b"c");
    akis.tuket(9);
    assert!(akis.veri().is_empty());
}
