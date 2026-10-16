//! `04 38` hasar, `05 38` periyodik, `1B 92` HP gövde çözümü.
//! Bayt tablosu `protokol-mesajlar.md` içindedir.

use super::{Hasar, Hp, Periyodik};

#[path = "hasar_alan.rs"]
mod alan;
use alan::Okuyucu;

pub fn hasar(govde: &[u8]) -> Option<Hasar> {
    let mut o = Okuyucu::yeni(govde);
    let hedef = o.varint()?;
    let anahtar = o.u8()?;
    o.varint()?;
    let actor = o.varint()?;
    let beceri_ham = o.u32()?;
    o.u8()?;
    let vurus_turu = o.u8()?;
    let mut sonuc = Hasar {
        hedef,
        actor,
        beceri_ham,
        vurus_turu,
        anahtar,
        deger: 0,
        ek_toplam: 0,
        ek_sayisi: 0,
        plotter: 0,
        aci: 0,
    };

    // Bilinmeyen bitler değişken kuyruğu etkileyebilir; sabit başlık yine değerlidir.
    if anahtar & !0x26 != 0 {
        return Some(sonuc);
    }
    if anahtar & 0x02 != 0 {
        sonuc.plotter = o.u8()?;
        o.varint()?;
        sonuc.aci = o.u8()?;
    }
    o.u32()?;
    o.u32()?;
    o.varint()?;
    if anahtar & 0x04 != 0 {
        sonuc.deger = u64::from(o.varint()?);
    }
    if anahtar & 0x20 != 0 {
        sonuc.ek_sayisi = o.u8()?;
        for _ in 0..sonuc.ek_sayisi {
            sonuc.ek_toplam = sonuc.ek_toplam.checked_add(u64::from(o.varint()?))?;
        }
    }
    o.u8()?;
    o.u8()?;
    o.bitti().then_some(sonuc)
}

pub fn periyodik(govde: &[u8]) -> Option<Periyodik> {
    let mut o = Okuyucu::yeni(govde);
    let hedef = o.varint()?;
    let etki_turu = o.u8()?;
    let actor = o.varint()?;
    o.varint()?;
    let beceri_ham = o.u32()?;
    // Kayıt: heal/HoT/DoT'ta miktar varinti, ardından türe göre 1-2 varint, en sonda bir bayt.
    // Status (00) ve kısa buff gövdeleri beceriyle biter.
    let miktar = if o.bitti() { 0 } else { u64::from(o.varint()?) };
    while o.kalan() > 1 {
        o.varint()?;
    }
    if !o.bitti() {
        o.u8()?;
    }
    o.bitti().then_some(Periyodik {
        hedef,
        etki_turu,
        actor,
        beceri_ham,
        miktar,
    })
}

pub fn hp(govde: &[u8]) -> Option<Hp> {
    let mut o = Okuyucu::yeni(govde);
    let actor = o.varint()?;
    // Kayıt (2026-10-16): actor'dan sonra iki varint; ilk kayıtta ikisi de 2 bayt olduğundan u32 sanılmıştı.
    o.varint()?;
    o.varint()?;
    let mevcut = o.u32()?;
    let azami = o.u32()?;
    o.atla(8)?;
    o.u32()?;
    o.u32()?;
    o.u8()?;
    (o.bitti() && azami != 0 && mevcut <= azami).then_some(Hp {
        actor,
        mevcut,
        azami,
    })
}

#[cfg(test)]
#[path = "hasar_testler.rs"]
mod testler;
#[cfg(test)]
#[path = "hasar_testler_hp.rs"]
mod testler_hp;
