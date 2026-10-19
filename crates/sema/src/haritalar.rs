//! Harita kimliği -> ad. Yerleşik tablo (`veri/haritalar.toml`, topluluk verisi, `veri/KAYNAK.md`) + kullanıcı
//! dosyası `%APPDATA%\CanMeter\haritalar.toml` (uygulama bilinmeyen bölgede sorar, cevap buraya yazılır; kullanıcı
//! satırı yerleşiği ezer). Biçim TOML alt kümesi, elle ayrıştırılır: `600032 = "Draupnir"`, `#` yorum, boş satır,
//! `[...]` başlık satırı yok sayılır. Çift tırnak isteğe bağlı; tırnak içinde `\"` ve `\\` kaçışı, kapanıştan sonrası
//! (ör. ` # not`) yok sayılır. Dosya işleri (`kaydet`, `kullanici_yolu`) `haritalar_dosya.rs` içindedir.
use std::collections::HashMap;

#[path = "haritalar_dosya.rs"]
mod dosya;
use dosya::temizle;
pub use dosya::{kaydet, kullanici_yolu};

pub const YERLESIK: &str = include_str!("../veri/haritalar.toml");
/// Bir adın en çok karakter sayısı (`kaydet` ve `ekle` kırpar).
pub const AD_TAVANI: usize = 60;

#[derive(Debug, Default, Clone)]
pub struct Haritalar(HashMap<u32, String>);

/// Tırnaklıysa ilk kaçışsız kapanış tırnağına kadar içini alır ve kaçışları çözer; kapanmıyorsa ham metin.
/// Tırnaksızsa olduğu gibi (satır sonuna kadar).
fn deger(ham: &str) -> String {
    let ham = ham.trim();
    let Some(govde) = ham.strip_prefix('"') else {
        return ham.to_string();
    };
    let mut cikti = String::new();
    let mut kacis = false;
    for c in govde.chars() {
        if kacis {
            cikti.push(c);
            kacis = false;
        } else if c == '\\' {
            kacis = true;
        } else if c == '"' {
            return cikti;
        } else {
            cikti.push(c);
        }
    }
    ham.to_string()
}

impl Haritalar {
    /// Yalnız yerleşik tablo.
    pub fn yerlesik() -> Self {
        let mut h = Self::default();
        h.ayristir(YERLESIK);
        h
    }

    /// Yerleşik + kullanıcı dosyası (varsa; okunamazsa sessizce yalnız yerleşik). Kullanıcı satırı yerleşiği ezer.
    pub fn yukle() -> Self {
        let mut h = Self::yerlesik();
        if let Some(metin) = kullanici_yolu().and_then(|y| std::fs::read_to_string(y).ok()) {
            h.ayristir(&metin);
        }
        h
    }

    /// Satırları işler: bozuk satır atlanır, boş ad atlanır, aynı kimlik tekrar gelirse son kazanır.
    pub fn ayristir(&mut self, metin: &str) {
        let satirlar = metin.lines().map(str::trim);
        for satir in satirlar.filter(|s| !s.is_empty() && !s.starts_with(['#', '['])) {
            if let Some((sol, sag)) = satir.split_once('=')
                && let Ok(kimlik) = sol.trim().parse::<u32>()
            {
                self.ekle(kimlik, &deger(sag));
            }
        }
    }

    pub fn ad(&self, harita: u32) -> Option<&str> {
        self.0.get(&harita).map(String::as_str)
    }

    /// Ad ya da "Bölge #kimlik".
    pub fn goruntu(&self, harita: u32) -> String {
        self.ad(harita)
            .map_or_else(|| format!("Bölge #{harita}"), str::to_string)
    }

    /// Belleğe ekler (dosyaya yazmaz); ad `trim` + AD_TAVANI karakterde kırpılır; boşsa etkisiz.
    pub fn ekle(&mut self, harita: u32, ad: &str) {
        let ad = temizle(ad);
        if !ad.is_empty() {
            self.0.insert(harita, ad);
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
#[path = "haritalar_testler.rs"]
mod testler;
