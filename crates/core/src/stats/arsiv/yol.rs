//! Encounter dosya yolu ve satır yazımı: `.partial` önce, tamamlanınca yeniden adlandırma.
use canmeter_sema::Kayit;
use chrono::{DateTime, Utc};
use std::fs;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

/// Aynı saniyede en çok dosya sırası.
const EN_COK_SIRA: u32 = 99;

/// `kok/YYYY-MM/YYYYMMDD-HHMMSS-NN.cmenc.jsonl` (UTC); klasörü oluşturur; aynı ad varsa NN artar (01..99).
pub(super) fn hedef_yol(kok: &Path, baslangic_ms: u64) -> Result<(PathBuf, String), String> {
    let zaman = DateTime::<Utc>::from_timestamp_millis(baslangic_ms as i64)
        .ok_or_else(|| format!("geçersiz zaman: {baslangic_ms}"))?;
    let klasor = kok.join(zaman.format("%Y-%m").to_string());
    fs::create_dir_all(&klasor).map_err(|e| format!("{}: {e}", klasor.display()))?;
    let on = zaman.format("%Y%m%d-%H%M%S").to_string();
    for n in 1..=EN_COK_SIRA {
        let kimlik = format!("{on}-{n:02}");
        let yol = klasor.join(format!("{kimlik}.cmenc.jsonl"));
        if !yol.exists() && !gecici(&yol).exists() {
            return Ok((yol, kimlik));
        }
    }
    Err(format!("{}: {on} için sıra kalmadı", klasor.display()))
}

/// Yazım sırasındaki ad: `<yol>.partial`.
pub(super) fn gecici(yol: &Path) -> PathBuf {
    let mut ad = yol.as_os_str().to_owned();
    ad.push(".partial");
    PathBuf::from(ad)
}

/// Kayıtları `.partial` dosyasına JSON satırı olarak yazar, sonra hedef ada taşır; hatada yarım dosya silinir.
pub(super) fn yaz_kayitlar(yol: &Path, kayitlar: &[Kayit]) -> Result<(), String> {
    let gecici = gecici(yol);
    let sonuc = yaz_tasi(&gecici, yol, kayitlar);
    if sonuc.is_err() {
        let _ = fs::remove_file(&gecici);
    }
    sonuc.map_err(|e| format!("{}: {e}", gecici.display()))
}

fn yaz_tasi(gecici: &Path, yol: &Path, kayitlar: &[Kayit]) -> std::io::Result<()> {
    let mut w = BufWriter::new(fs::File::create(gecici)?);
    for k in kayitlar {
        w.write_all(serde_json::to_string(k)?.as_bytes())?;
        w.write_all(b"\n")?;
    }
    w.flush()?;
    drop(w);
    fs::rename(gecici, yol)
}
