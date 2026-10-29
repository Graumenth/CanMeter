use crate::sema::{Beceriler, Encounter, Haritalar, Kayit, Kosu};
use crate::{kosu::KosuOzeti, oku_tara::tara};
use chrono::DateTime;
use std::{
    cell::OnceCell,
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Default)]
pub struct Gecmis {
    pub savaslar: Vec<Encounter>,
    pub kosular: Vec<Kosu>,
    pub haritalar: Haritalar,
    pub beceriler: Beceriler,
    pub kosu_ozetleri: OnceCell<Vec<KosuOzeti>>,
}

pub fn gecmis(yol: &Path) -> Result<Gecmis, String> {
    Ok(Gecmis {
        savaslar: klasor(yol)?,
        kosular: kosular(yol)?,
        haritalar: Haritalar::yukle(),
        beceriler: Beceriler::yukle(),
        kosu_ozetleri: OnceCell::new(),
    })
}

pub fn kosular(yol: &Path) -> Result<Vec<Kosu>, String> {
    let mut kosular = Vec::new();
    tara(yol, &mut |yol| {
        if yol.file_name().is_some_and(|ad| ad == "bolgeler.jsonl") {
            match fs::File::open(yol) {
                Ok(giris) => {
                    for satir in BufReader::new(giris).split(b'\n') {
                        let Ok(baytlar) = satir else { break };
                        if let Ok(Kayit::Kosu(k)) = serde_json::from_slice(&baytlar) {
                            kosular.push(k);
                        }
                    }
                }
                Err(h) => eprintln!("Uyarı: {} açılamadı: {h}", yol.display()),
            }
        }
        Ok(())
    })?;
    kosular
        .sort_by_cached_key(|k| std::cmp::Reverse(DateTime::parse_from_rfc3339(&k.giris_utc).ok()));
    Ok(kosular)
}

pub fn klasor(yol: &Path) -> Result<Vec<Encounter>, String> {
    let mut savaslar = Vec::new();
    tara(yol, &mut |yol| {
        if yol.to_string_lossy().ends_with(".cmenc.jsonl")
            && let Some(savas) = dosya(yol)?
        {
            savaslar.push(savas);
        }
        Ok(())
    })?;
    savaslar.sort_by_cached_key(|e| {
        std::cmp::Reverse(DateTime::parse_from_rfc3339(&e.baslik.baslangic_utc).ok())
    });
    Ok(savaslar)
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
            Some(Kayit::Bilinmeyen | Kayit::Kosu(_)) => {}
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
