use std::{fs, path::Path};

pub fn yaz(cikti: &Path) -> Result<(), String> {
    let ikon = cikti.join("ikon");
    fs::create_dir_all(&ikon).map_err(|h| format!("İkon klasörü oluşturulamadı: {h}"))?;
    for (ad, baytlar) in [
        (
            "gladiator",
            include_bytes!("../sablon/ikon/gladiator.png").as_slice(),
        ),
        (
            "templar",
            include_bytes!("../sablon/ikon/templar.png").as_slice(),
        ),
        (
            "assassin",
            include_bytes!("../sablon/ikon/assassin.png").as_slice(),
        ),
        (
            "ranger",
            include_bytes!("../sablon/ikon/ranger.png").as_slice(),
        ),
        (
            "sorcerer",
            include_bytes!("../sablon/ikon/sorcerer.png").as_slice(),
        ),
        (
            "spiritmaster",
            include_bytes!("../sablon/ikon/spiritmaster.png").as_slice(),
        ),
        (
            "cleric",
            include_bytes!("../sablon/ikon/cleric.png").as_slice(),
        ),
        (
            "chanter",
            include_bytes!("../sablon/ikon/chanter.png").as_slice(),
        ),
    ] {
        fs::write(ikon.join(format!("{ad}.png")), baytlar)
            .map_err(|h| format!("{ad} ikonu yazılamadı: {h}"))?;
    }
    for (ad, metin) in [
        ("stil.css", include_str!("../sablon/stil.css")),
        ("betik.js", include_str!("../sablon/betik.js")),
    ] {
        fs::write(cikti.join(ad), metin).map_err(|h| format!("{ad} yazılamadı: {h}"))?;
    }
    Ok(())
}
