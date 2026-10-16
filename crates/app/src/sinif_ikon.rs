//! Gerçek sınıf ikonları: oyun içi 40x42 ikonlar ham RGBA olarak gömülü (`ikonlar/*.rgba`, çözücü kütüphane yok,
//! 8 x 6720 bayt). Telif NC Corporation, GPL kapsamı dışı (bkz. `ikonlar/KAYNAK.md`). Dokular açılışta bir kez
//! yüklenir; skin'ler `ciz` ile çizer, `false` dönerse (Bilinmiyor) kendi şeklini çizer.

use canmeter_core::stats::Sinif;
use egui::{
    Color32, ColorImage, Context, Painter, Rect, TextureHandle, TextureOptions, pos2, vec2,
};
use std::sync::OnceLock;

const BOYUT: [usize; 2] = [40, 42];
const SINIFLAR: [Sinif; 8] = [
    Sinif::Gladiator,
    Sinif::Templar,
    Sinif::Assassin,
    Sinif::Ranger,
    Sinif::Sorcerer,
    Sinif::Spiritmaster,
    Sinif::Cleric,
    Sinif::Chanter,
];
static DOKULAR: OnceLock<Vec<(Sinif, TextureHandle)>> = OnceLock::new();

fn ham(sinif: Sinif) -> Option<&'static [u8]> {
    Some(match sinif {
        Sinif::Gladiator => include_bytes!("../ikonlar/gladiator.rgba"),
        Sinif::Templar => include_bytes!("../ikonlar/templar.rgba"),
        Sinif::Assassin => include_bytes!("../ikonlar/assassin.rgba"),
        Sinif::Ranger => include_bytes!("../ikonlar/ranger.rgba"),
        Sinif::Sorcerer => include_bytes!("../ikonlar/sorcerer.rgba"),
        Sinif::Spiritmaster => include_bytes!("../ikonlar/spiritmaster.rgba"),
        Sinif::Cleric => include_bytes!("../ikonlar/cleric.rgba"),
        Sinif::Chanter => include_bytes!("../ikonlar/chanter.rgba"),
        Sinif::Bilinmiyor => return None,
    })
}

/// Açılışta bir kez (`Uygulama::new`); sonraki çağrılar etkisiz.
pub fn yukle(ctx: &Context) {
    DOKULAR.get_or_init(|| {
        SINIFLAR
            .iter()
            .filter_map(|&sinif| {
                let resim = ColorImage::from_rgba_unmultiplied(BOYUT, ham(sinif)?);
                let ad = format!("sinif-{}", sinif.ad());
                Some((sinif, ctx.load_texture(ad, resim, TextureOptions::LINEAR)))
            })
            .collect()
    });
}

/// İkonu `alan` içine en-boy oranını koruyarak ortalar ve çizer; doku yoksa `false` (skin kendi şeklini çizer).
pub fn ciz(p: &Painter, alan: Rect, sinif: Sinif, ton: Color32) -> bool {
    let Some((_, doku)) = DOKULAR
        .get()
        .and_then(|d| d.iter().find(|(s, _)| *s == sinif))
    else {
        return false;
    };
    let oran = BOYUT[0] as f32 / BOYUT[1] as f32;
    let boy = alan.height().min(alan.width() / oran);
    let hedef = Rect::from_center_size(alan.center(), vec2(boy * oran, boy));
    let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    p.image(doku.id(), hedef, uv, ton);
    true
}
