//! NPC / summon / efekt varlıkları: 41 36 ile spawn olanlar asla oyuncu değildir. Sahibi bilinenler sahibine
//! bağlanır (hasar ve iyileştirme sahibin satırına yazılır), sahipsizler satır almaz (mob ya da sahipsiz efekt).

use super::{Tablo, Varlik};
use crate::protocol::mesaj::NpcSpawn;
use crate::stats::Sinif;

impl Tablo {
    pub(super) fn npc_spawn(&mut self, s: &NpcSpawn) {
        // Sahip: önce kimlik, yoksa ad (summon ve efektler sahibinin adını taşır).
        let sahip = s
            .sahip
            .or_else(|| s.sahip_adi.as_deref().and_then(|ad| self.oyuncu_id(ad)));
        self.npcler.insert(s.id);
        let v = self.varliklar.entry(s.id).or_insert(Varlik {
            ad: String::new(),
            sinif: Sinif::Bilinmiyor,
            oyuncu: false,
            olu: false,
        });
        v.oyuncu = false;
        v.olu = false;
        match sahip {
            Some(k) if k != s.id => {
                self.sahipler.insert(s.id, k);
            }
            _ => {
                self.sahipler.remove(&s.id);
            }
        }
    }

    pub(super) fn oyuncu_id(&self, ad: &str) -> Option<u32> {
        self.varliklar
            .iter()
            .find(|(_, v)| v.oyuncu && v.ad == ad)
            .map(|(&id, _)| id)
    }

    /// Aynı adlı başka bir oyuncu kimliğinin kökü (`haric` dışında). Takma kimlik yeniden spawn olunca kendisini
    /// bulup bağı düşürüyordu (0.6.10): HashMap sırası rastgele, `oyuncu_id` kendisini döndürebiliyordu.
    pub(super) fn ad_koku(&self, ad: &str, haric: u32) -> Option<u32> {
        self.varliklar
            .iter()
            .find(|(id, v)| **id != haric && v.oyuncu && v.ad == ad)
            .map(|(id, _)| self.kok(*id))
            .filter(|&k| k != haric)
    }

    /// Sahip zincirinin kökü (en çok 4 adım); sahipsizse kendisi.
    pub fn kok(&self, id: u32) -> u32 {
        let mut k = id;
        for _ in 0..4 {
            match self.sahipler.get(&k) {
                Some(&s) if s != k => k = s,
                _ => break,
            }
        }
        k
    }

    /// 41 36 ile spawn olmuş (oyuncu olmayan) varlık.
    pub fn npc_mi(&self, id: u32) -> bool {
        self.npcler.contains(&id)
    }
}
