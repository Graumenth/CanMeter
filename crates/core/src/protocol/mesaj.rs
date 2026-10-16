//! Mesaj çözümü: (tür, gövde) -> anlamlı kayıt. Bayt düzeni `CanMeterCC/protokol-mesajlar.md` (kayıttan),
//! anlam `protokol.md` §5-7. Sıcak yolda ayırma yok: yalnız ad taşıyan seyrek mesajlar `String` kurar.
//! Çözülemeyen ya da ilgisiz tür `None`; çağıran saymaya devam eder.

pub mod hasar;
pub mod kimlik;
pub mod spawn_npc;

pub use spawn_npc::NpcSpawn;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mesaj {
    Hasar(Hasar),
    Periyodik(Periyodik),
    Hp(Hp),
    /// `33 36`: yerel oyuncu kimliği.
    Kimlik(Kimlik),
    /// `45 36` (eski `44 36`): oyuncu spawn, yetkili ad kaynağı.
    OyuncuSpawn(OyuncuSpawn),
    /// `02 97`: parti listesi.
    Roster(Roster),
    /// `42 36` (eski `41 36`): ölüm ya da despawn.
    Olum(Olum),
    /// `41 36` (eski `40 36`): NPC, summon, pet ya da skill-efekt varlığı; summon ve efektte sahip bilgisi.
    NpcSpawn(NpcSpawn),
}

/// `04 38` doğrudan hasar. `deger` vuruşun toplamıdır; `ek_*` alt vuruşlar (toplama dahil).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hasar {
    pub hedef: u32,
    pub actor: u32,
    pub beceri_ham: u32,
    /// Spec §6: 1 miss, 2 normal, 3 kritik, 6 resist.
    pub vurus_turu: u8,
    /// Switch baytı (hangi ek alanların geldiğini belirler); hata ayıklama için saklanır.
    pub anahtar: u8,
    pub deger: u64,
    pub ek_toplam: u64,
    pub ek_sayisi: u8,
    /// Plotter bitleri (§6) ve açı; alan yoksa 0.
    pub plotter: u8,
    pub aci: u8,
}

/// `05 38` periyodik etki: DoT, HoT, heal, buff tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Periyodik {
    pub hedef: u32,
    /// §6: 01/09 normal heal, 0B HoT, 02/0A DoT, 00 status, 08 buff.
    pub etki_turu: u8,
    pub actor: u32,
    pub beceri_ham: u32,
    pub miktar: u64,
}

/// `1B 92` HP güncellemesi: actor varint, iki varint, mevcut ve azami u32 (2026-10-16 kaydıyla doğrulandı).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hp {
    pub actor: u32,
    pub mevcut: u32,
    pub azami: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kimlik {
    pub id: u32,
    pub ad: String,
    pub sunucu: u16,
    pub sinif_kodu: u32,
    pub seviye: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OyuncuSpawn {
    pub id: u32,
    pub ad: String,
    pub sinif_kodu: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RosterUye {
    pub ad: String,
    pub sinif_kodu: u32,
    pub seviye: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Roster {
    pub uyeler: Vec<RosterUye>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Olum {
    pub id: u32,
    /// §6: 1 zone başında ölü, 3 savaş ölümü, 7 despawn.
    pub bayrak: u8,
}

/// Türe göre dağıtır. Bilinmeyen tür ya da bozuk gövde `None`.
pub fn coz(tur: [u8; 2], govde: &[u8]) -> Option<Mesaj> {
    match tur {
        [0x04, 0x38] => hasar::hasar(govde).map(Mesaj::Hasar),
        [0x05, 0x38] => hasar::periyodik(govde).map(Mesaj::Periyodik),
        [0x1B, 0x92] => hasar::hp(govde).map(Mesaj::Hp),
        [0x33, 0x36] => kimlik::kimlik(govde).map(Mesaj::Kimlik),
        [0x45, 0x36] | [0x44, 0x36] => kimlik::oyuncu_spawn(govde).map(Mesaj::OyuncuSpawn),
        [0x02, 0x97] => kimlik::roster(govde).map(Mesaj::Roster),
        [0x42, 0x36] => kimlik::olum(govde).map(Mesaj::Olum),
        [0x41, 0x36] | [0x40, 0x36] => spawn_npc::npc_spawn(govde).map(Mesaj::NpcSpawn),
        _ => None,
    }
}
