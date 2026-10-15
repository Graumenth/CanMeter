//! Kayıttaki sınıf kodlarını sayar (yalnız kod ve eşlenen sınıf; ad basılmaz).

use std::collections::BTreeMap;

use canmeter_core::protocol::mesaj::{Mesaj, kimlik::sinif};

#[derive(Default)]
pub struct SinifSayaci {
    kodlar: BTreeMap<(u32, &'static str), u32>,
}

impl SinifSayaci {
    pub fn say(&mut self, mesaj: &Mesaj) {
        let kodlar: Vec<(u32, &'static str)> = match mesaj {
            Mesaj::Kimlik(k) => vec![(k.sinif_kodu, "kimlik")],
            Mesaj::OyuncuSpawn(s) => vec![(s.sinif_kodu, "spawn")],
            Mesaj::Roster(r) => r.uyeler.iter().map(|u| (u.sinif_kodu, "roster")).collect(),
            _ => return,
        };
        for kod in kodlar {
            *self.kodlar.entry(kod).or_default() += 1;
        }
    }

    pub fn yaz(&self) {
        for ((kod, kaynak), adet) in &self.kodlar {
            println!(
                "sınıf kodu {kod} ({kaynak}): {adet} kez -> {}",
                sinif(*kod).ad()
            );
        }
    }
}
