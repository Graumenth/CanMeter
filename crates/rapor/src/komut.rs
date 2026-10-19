use crate::{oku, uret};
use std::{env, ffi::OsString, path::PathBuf};

pub struct Secenekler {
    pub girdi: PathBuf,
    pub cikti: PathBuf,
    pub ac: bool,
}

pub fn cozumle(sozler: impl IntoIterator<Item = OsString>) -> Result<Secenekler, String> {
    let mut girdi = None;
    let mut cikti = None;
    let mut ac = false;
    let mut sozler = sozler.into_iter();
    while let Some(soz) = sozler.next() {
        match soz.to_str() {
            Some("--ac") => ac = true,
            Some("--girdi" | "--cikti") => {
                let yol = sozler
                    .next()
                    .filter(|s| !s.is_empty() && !s.to_string_lossy().starts_with("--"))
                    .ok_or_else(|| format!("{} için klasör yolu gerekli", soz.to_string_lossy()))?;
                if soz == "--girdi" {
                    girdi = Some(PathBuf::from(yol));
                } else {
                    cikti = Some(PathBuf::from(yol));
                }
            }
            _ => {
                return Err(format!(
                    "Bilinmeyen seçenek: {}. Kullanım: canmeter-rapor [--girdi <klasör>] [--cikti <klasör>] [--ac]",
                    soz.to_string_lossy()
                ));
            }
        }
    }
    let varsayilan = || {
        env::var_os("LOCALAPPDATA")
            .map(|p| PathBuf::from(p).join("CanMeter"))
            .ok_or_else(|| "LOCALAPPDATA tanımlı değil; --girdi ve --cikti belirtin".to_string())
    };
    Ok(Secenekler {
        girdi: match girdi {
            Some(yol) => yol,
            None => varsayilan()?.join("encounters"),
        },
        cikti: match cikti {
            Some(yol) => yol,
            None => varsayilan()?.join("rapor"),
        },
        ac,
    })
}

pub fn calistir() -> Result<(), String> {
    let secenek = cozumle(env::args_os().skip(1))?;
    let gecmis = oku::gecmis(&secenek.girdi)?;
    uret::yaz(&gecmis, &secenek.cikti)?;
    println!(
        "{} savaş raporu yazıldı: {}",
        gecmis.savaslar.len(),
        secenek.cikti.display()
    );
    if secenek.ac {
        ac(secenek.cikti.join("index.html"))?;
    }
    Ok(())
}

#[cfg(windows)]
fn ac(yol: PathBuf) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    let yol = yol
        .canonicalize()
        .map_err(|h| format!("Dizin yolu çözülemedi: {h}"))?;
    let metin = yol.to_string_lossy();
    let metin = metin.strip_prefix(r"\\?\").map_or(metin.as_ref(), |s| s);
    // cmd değişken genişletmesi yolun anlamını değiştirmemeli.
    if metin.contains(['%', '!', '"', '\r', '\n']) {
        return Err("Tarayıcı yolu %, !, tırnak veya satır sonu içeremez".into());
    }
    let sonuc = std::process::Command::new("cmd")
        .args(["/d", "/v:off", "/c"])
        .raw_arg(format!(r#"start "" "{metin}""#))
        .output()
        .map_err(|h| format!("Tarayıcı açılamadı: {h}"))?;
    if sonuc.status.success() {
        Ok(())
    } else {
        Err("Tarayıcı başlatılamadı".into())
    }
}

#[cfg(not(windows))]
fn ac(_: PathBuf) -> Result<(), String> {
    Err("--ac yalnız Windows üzerinde kullanılabilir".into())
}
