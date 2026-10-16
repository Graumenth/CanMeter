//! Kayıttaki sınıf kodlarını sayar (yalnız kod ve eşlenen sınıf; ad basılmaz).

use std::collections::BTreeMap;

use canmeter_core::protocol::mesaj::{Mesaj, kimlik::sinif};

#[derive(Default)]
pub struct SinifSayaci {
    kodlar: BTreeMap<(u32, &'static str), u32>,
    /// 41 36 tür -> (adet, sahip kimlikli, sahip adlı, HP'li).
    npc: BTreeMap<u8, (u32, u32, u32, u32)>,
}

impl SinifSayaci {
    pub fn say(&mut self, mesaj: &Mesaj) {
        let kodlar: Vec<(u32, &'static str)> = match mesaj {
            Mesaj::Kimlik(k) => vec![(k.sinif_kodu, "kimlik")],
            Mesaj::OyuncuSpawn(s) => vec![(s.sinif_kodu, "spawn")],
            Mesaj::Roster(r) => r.uyeler.iter().map(|u| (u.sinif_kodu, "roster")).collect(),
            Mesaj::NpcSpawn(s) => {
                let e = self.npc.entry(s.tur).or_default();
                e.0 += 1;
                e.1 += u32::from(s.sahip.is_some());
                e.2 += u32::from(s.sahip_adi.is_some());
                e.3 += u32::from(s.hp.is_some());
                return;
            }
            _ => return,
        };
        for kod in kodlar {
            *self.kodlar.entry(kod).or_default() += 1;
        }
    }

    pub fn yaz(&self) {
        for (tur, (adet, sahipli, adli, hpli)) in &self.npc {
            println!(
                "npc spawn tür {tur:02X}: {adet} (sahip kimliği {sahipli}, sahip adı {adli}, HP {hpli})"
            );
        }
        for ((kod, kaynak), adet) in &self.kodlar {
            println!(
                "sınıf kodu {kod} ({kaynak}): {adet} kez -> {}",
                sinif(*kod).ad()
            );
        }
    }
}
