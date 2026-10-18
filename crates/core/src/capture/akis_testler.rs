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
#[test]
fn ileri_mi_yeniden_iletim_ve_keepalive_false() {
    let mut akis = Akis::new();
    assert!(akis.ileri_mi(5)); // taban yokken her seq ileridir
    akis.ekle(100, b"abcd");
    assert!(akis.ileri_mi(104)); // tam sıradaki
    assert!(akis.ileri_mi(200)); // ilerideki (sıra dışı) segment de ileridir
    assert!(!akis.ileri_mi(103)); // keepalive: seq - 1
    assert!(!akis.ileri_mi(100)); // yeniden iletim
    let mut sarmal = Akis::new();
    sarmal.ekle(u32::MAX - 1, b"ab");
    assert!(sarmal.ileri_mi(0)); // sarma sonrası ileri
    assert!(!sarmal.ileri_mi(u32::MAX - 1)); // sarma öncesi tekrar
}
