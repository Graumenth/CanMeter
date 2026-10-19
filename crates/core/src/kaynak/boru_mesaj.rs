//! Çözülmüş mesajı tabloya ve toplayıcıya uygular; Boru'nun sıcak yolu (olay başına O(1)).
use crate::combat::Tablo;
use crate::protocol::mesaj::{Hp, Mesaj};
use crate::stats::toplayici::Toplayici;

/// Summon ve efekt hasarı sahibine yazılır; ölüm yalnız oyuncular için sayılır.
pub(super) fn isle(tablo: &mut Tablo, toplayici: &mut Toplayici, zaman_ms: u64, mesaj: &Mesaj) {
    toplayici.ilerle(zaman_ms, tablo);
    tablo.isle(mesaj);
    match mesaj {
        Mesaj::Hasar(h) => {
            let mut h = *h;
            h.actor = tablo.kok(h.actor);
            let dost = tablo.partide(h.actor) && tablo.partide(h.hedef);
            toplayici.hasar(zaman_ms, &h, dost);
        }
        Mesaj::Periyodik(p) => {
            let mut p = *p;
            p.actor = tablo.kok(p.actor);
            toplayici.periyodik(zaman_ms, &p);
        }
        Mesaj::Hp(h) => {
            // Ölü bilinen varlık HP > 0 ile dirildi: yaşam olayı (satır kökü).
            let dirildi = h.mevcut > 0 && tablo.varlik(h.actor).is_some_and(|v| v.olu);
            tablo.dirilt(h.actor, h.mevcut);
            toplayici.hp(h);
            if dirildi {
                toplayici.dirildi(tablo.kok(h.actor));
            }
        }
        Mesaj::Olum(o) if o.bayrak == 3 => {
            toplayici.hedef_oldu(o.id);
            if !tablo.npc_mi(o.id) {
                toplayici.olum(tablo.kok(o.id));
            }
        }
        Mesaj::NpcSpawn(s) => {
            if let Some((mevcut, azami)) = s.hp {
                toplayici.hp(&Hp {
                    actor: s.id,
                    mevcut,
                    azami,
                });
            }
        }
        _ => {}
    }
}
