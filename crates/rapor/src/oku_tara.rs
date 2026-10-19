use std::{fs, path::Path};

pub(crate) fn tara(
    yol: &Path,
    oku: &mut dyn FnMut(&Path) -> Result<(), String>,
) -> Result<(), String> {
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
            tara(&yol, oku)?;
        } else if bilgi.is_file() {
            oku(&yol)?;
        }
    }
    Ok(())
}
