use crate::{
    html::{dizin, savas},
    sema::Encounter,
    varlik,
};
use std::{collections::HashSet, fs, path::Path};

pub fn yaz(savaslar: &[Encounter], cikti: &Path) -> Result<(), String> {
    let mut kimlikler = HashSet::new();
    for e in savaslar {
        // Aynı çıktı yoluna sahip kayıtları sessizce ezmeyiz.
        let kimlik = crate::html::serit::dosya_kimligi(&e.baslik.kimlik);
        if !kimlikler.insert(kimlik.to_ascii_lowercase()) {
            return Err(format!("Tekrarlanan savaş kimliği: {}", e.baslik.kimlik));
        }
    }
    fs::create_dir_all(cikti.join("savas"))
        .map_err(|h| format!("{} oluşturulamadı: {h}", cikti.display()))?;
    varlik::yaz(cikti)?;
    for (indeks, e) in savaslar.iter().enumerate() {
        let kimlik = crate::html::serit::dosya_kimligi(&e.baslik.kimlik);
        let yol = cikti.join("savas").join(format!("{kimlik}.html"));
        fs::write(&yol, savas::yaz(savaslar, indeks))
            .map_err(|h| format!("{} yazılamadı: {h}", yol.display()))?;
    }
    fs::write(cikti.join("index.html"), dizin::yaz(savaslar))
        .map_err(|h| format!("Dizin sayfası yazılamadı: {h}"))
}
