//! Kalıcı kullanıcı ayarları; okunamayan kayıt varsayılanlarla açılır.

use crate::{
    dil::Dil,
    skin::{Zemin, kayit},
};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Ayarlar {
    pub dil: Dil,
    pub skin: String,
    pub zemin_renk: Option<[u8; 3]>,
    pub zemin_opaklik: f32,
    pub toplam_goster: bool,
    /// Overlay ölçeği (1.0 = 440 px genişlik); kenardan sürükleyince değişir.
    pub olcek: f32,
    /// Pencere dış köşesi (doğal nokta); taşıyınca güncellenir, açılışta kullanılır.
    pub konum: Option<[i32; 2]>,
}

impl Default for Ayarlar {
    fn default() -> Self {
        Self {
            dil: Dil::sistem(),
            skin: kayit::VARSAYILAN.into(),
            zemin_renk: None,
            zemin_opaklik: 0.92,
            toplam_goster: true,
            olcek: 1.0,
            konum: None,
        }
    }
}

impl Ayarlar {
    pub fn yukle() -> Self {
        let ayarlar: Self = yol()
            .and_then(|yol| fs::read_to_string(yol).ok())
            .and_then(|metin| toml::from_str(&metin).ok())
            .unwrap_or_default();
        if !ayarlar.zemin_opaklik.is_finite()
            || !(0.3..=1.0).contains(&ayarlar.zemin_opaklik)
            || !(0.5..=3.0).contains(&ayarlar.olcek)
        {
            return Self::default();
        }
        ayarlar
    }

    pub fn kaydet(&self) {
        let sonuc = (|| -> Result<(), Box<dyn std::error::Error>> {
            let yol = yol().ok_or("APPDATA bulunamadı")?;
            let metin = toml::to_string_pretty(self)?;
            fs::create_dir_all(yol.parent().ok_or("Ayar klasörü bulunamadı")?)?;
            fs::write(yol, metin)?;
            Ok(())
        })();
        if let Err(hata) = sonuc {
            eprintln!("Ayarlar kaydedilemedi: {hata}");
        }
    }

    pub fn zemin(&self) -> Zemin {
        Zemin {
            renk: self
                .zemin_renk
                .map(|[r, g, b]| egui::Color32::from_rgb(r, g, b)),
            opaklik: self.zemin_opaklik,
        }
    }
}

fn yol() -> Option<PathBuf> {
    Some(
        PathBuf::from(std::env::var_os("APPDATA")?)
            .join("CanMeter")
            .join("ayarlar.toml"),
    )
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn toml_gidis_donus() {
        for ayarlar in [
            Ayarlar::default(),
            Ayarlar {
                dil: Dil::En,
                skin: "sis".into(),
                zemin_renk: Some([12, 34, 56]),
                zemin_opaklik: 0.3,
                toplam_goster: false,
                olcek: 1.5,
                konum: Some([120, -40]),
            },
        ] {
            let metin = toml::to_string(&ayarlar).unwrap();
            assert!(metin.contains(&format!("dil = \"{}\"", ayarlar.dil.kod())));
            assert_eq!(toml::from_str::<Ayarlar>(&metin).unwrap(), ayarlar);
        }
        assert!(toml::from_str::<Ayarlar>("dil = 'de'").is_err());
    }
}
