use crate::sema::{Encounter, Kayit};
use chrono::DateTime;
use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn klasor(yol: &Path) -> Result<Vec<Encounter>, String> {
    let mut savaslar = Vec::new();
    tara(yol, &mut savaslar)?;
    savaslar.sort_by_cached_key(|e| {
        std::cmp::Reverse(DateTime::parse_from_rfc3339(&e.baslik.baslangic_utc).ok())
    });
    Ok(savaslar)
}

fn tara(yol: &Path, savaslar: &mut Vec<Encounter>) -> Result<(), String> {
    let liste =
        fs::read_dir(yol).map_err(|h| format!("{} klasörü okunamadı: {h}", yol.display()))?;
    let mut yollar = liste
        .map(|g| g.map(|g| g.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|h| format!("{} klasörü taranamadı: {h}", yol.display()))?;
    yollar.sort();
    for yol in yollar {
        let bilgi =
            fs::symlink_metadata(&yol).map_err(|h| format!("{} okunamadı: {h}", yol.display()))?;
        if bilgi.file_type().is_symlink() || yol.to_string_lossy().ends_with(".partial") {
            continue;
        }
        if bilgi.is_dir() {
            tara(&yol, savaslar)?;
        } else if bilgi.is_file()
            && yol.to_string_lossy().ends_with(".cmenc.jsonl")
            && let Some(savas) = dosya(&yol)?
        {
            savaslar.push(savas);
        }
    }
    Ok(())
}

pub fn dosya(yol: &Path) -> Result<Option<Encounter>, String> {
    let giris = fs::File::open(yol).map_err(|h| format!("{} açılamadı: {h}", yol.display()))?;
    let mut savas = Encounter {
        dosya: yol.display().to_string(),
        ..Encounter::default()
    };
    let mut baslik_var = false;
    let mut notlar = Vec::new();
    for (indeks, satir) in BufReader::new(giris).split(b'\n').enumerate() {
        let baytlar = satir.map_err(|h| format!("{} okunamadı: {h}", yol.display()))?;
        let kayit = std::str::from_utf8(&baytlar)
            .ok()
            .and_then(|s| serde_json::from_str::<Kayit>(s).ok());
        match kayit {
            Some(Kayit::Baslik(b)) => {
                if b.sema > canmeter_sema::SEMA {
                    notlar.push(format!("şema {} bu sürümden yeni", b.sema));
                }
                savas.baslik = b;
                baslik_var = true;
            }
            Some(Kayit::Katilimci(k)) => savas.katilimcilar.push(k),
            Some(Kayit::Beceri(b)) => savas.beceriler.push(b),
            Some(Kayit::Kova(k)) => savas.kovalar.push(k),
            Some(Kayit::Yasam(y)) => savas.yasam.push(y),
            Some(Kayit::Son(s)) => savas.tamam = s.tamam,
            Some(Kayit::Bilinmeyen) => {}
            None => notlar.push(format!("satır {} okunamadı", indeks + 1)),
        }
    }
    if !baslik_var {
        eprintln!("Uyarı: {} atlandı: baslik kaydı yok", yol.display());
        return Ok(None);
    }
    savas.baslik.kalite.extend(notlar);
    Ok(Some(savas))
}
