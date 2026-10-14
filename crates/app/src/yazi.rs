//! Yazı tipleri: Windows'un kendi Segoe UI / Consolas dosyaları çalışma anında okunur, ikiliye gömülmez.
//! Dosya yoksa egui'nin gömülü yazı tipleri kalır. Skin'ler yalnız `aile(UI | KALIN | MONO)` kullanır.

use std::path::Path;
use std::sync::Arc;

use egui::{FontData, FontDefinitions, FontFamily};

pub const UI: &str = "ui";
pub const KALIN: &str = "ui-kalin";
pub const MONO: &str = "mono";

/// Skin'lerin `FontId::new(boyut, aile(UI))` ile kullandığı aile.
pub fn aile(ad: &'static str) -> FontFamily {
    FontFamily::Name(ad.into())
}

/// Üç aileyi kurar; sistem yazı tipi bulunursa listenin başına, egui yedekleri arkasına.
pub fn yukle(ctx: &egui::Context) {
    let mut t = FontDefinitions::default();
    let kaynaklar = [
        (UI, "segoeui.ttf", FontFamily::Proportional),
        (KALIN, "segoeuib.ttf", FontFamily::Proportional),
        (MONO, "consola.ttf", FontFamily::Monospace),
    ];
    for (ad, dosya, yedek) in kaynaklar {
        let mut liste = t.families.get(&yedek).cloned().unwrap_or_default();
        if let Some(veri) = sistem_yazi(dosya) {
            t.font_data
                .insert(ad.to_owned(), Arc::new(FontData::from_owned(veri)));
            liste.insert(0, ad.to_owned());
        }
        t.families.insert(aile(ad), liste);
    }
    ctx.set_fonts(t);
}

fn sistem_yazi(dosya: &str) -> Option<Vec<u8>> {
    let kok = std::env::var_os("WINDIR")?;
    std::fs::read(Path::new(&kok).join("Fonts").join(dosya)).ok()
}
