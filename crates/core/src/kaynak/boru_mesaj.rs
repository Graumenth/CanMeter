//! Çözülmüş mesajı tabloya ve toplayıcıya uygular; Boru'nun sıcak yolu (olay başına O(1)).
use crate::combat::Tablo;
use crate::protocol::mesaj::{Hp, Mesaj};
use crate::stats::toplayici::Toplayici;

/// Summon ve efekt hasarı sahibine yazılır; ölüm yalnız oyuncular için sayılır.
/// Savaşı yalnız partiyi ilgilendiren hasar (aktör ya da hedef partide) açar ve uzatır (0.8.10). Yerel oyuncu
/// henüz tanınmadıysa (33 36 gelmedi) süzgeç yok: eski davranış, her olay ilgili.
pub(super) fn isle(tablo: &mut Tablo, toplayici: &mut Toplayici, zaman_ms: u64, mesaj: &Mesaj) {
    toplayici.ilerle(zaman_ms, tablo);
    tablo.isle(mesaj);
    match mesaj {
        Mesaj::Hasar(h) => {
            let mut h = *h;
            h.actor = tablo.kok(h.actor);
            let dost = tablo.partide(h.actor) && tablo.partide(h.hedef);
            let ilgili = ilgili_mi(tablo, h.actor, h.hedef);
            toplayici.hasar_suz(zaman_ms, &h, dost, ilgili);
        }
        Mesaj::Periyodik(p) => {
            let mut p = *p;
            p.actor = tablo.kok(p.actor);
            let ilgili = ilgili_mi(tablo, p.actor, p.hedef);
            toplayici.periyodik_suz(zaman_ms, &p, ilgili);
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
        Mesaj::Harita(h) => toplayici.bolge_gir(h.harita, h.ornek, zaman_ms),
        _ => {}
    }
}

/// Olay bizi ilgilendiriyor mu: aktör (kök çözümlü) ya da hedef partide. Yerel oyuncu bilinmiyorsa her olay ilgili
/// (süzecek ölçüt yok; kimlik gelince süzgeç devreye girer).
fn ilgili_mi(tablo: &Tablo, actor: u32, hedef: u32) -> bool {
    tablo.yerel().is_none() || tablo.partide(actor) || tablo.partide(hedef)
}

#[cfg(test)]
#[path = "boru_testler_yabanci.rs"]
mod testler_yabanci;
