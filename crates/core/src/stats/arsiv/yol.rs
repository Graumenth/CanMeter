//! Encounter dosya yolu ve satır yazımı: `.partial` önce, tamamlanınca yeniden adlandırma.
use canmeter_sema::Kayit;
use chrono::{DateTime, SecondsFormat, Utc};
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
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

/// Milisaniyeden RFC 3339 UTC (saniye, `Z`); geçersizse boş.
pub(super) fn utc(ms: u64) -> String {
    let zaman = DateTime::<Utc>::from_timestamp_millis(ms as i64);
    zaman.map_or_else(String::new, |z| {
        z.to_rfc3339_opts(SecondsFormat::Secs, true)
    })
}

/// Koşu satırını `kok/YYYY-MM/bolgeler.jsonl` sonuna ekler (ay giriş zamanından, UTC); klasörü oluşturur.
pub(super) fn kosu_ekle(kok: &Path, giris_ms: u64, satir: &Kayit) -> Result<(), String> {
    let zaman = DateTime::<Utc>::from_timestamp_millis(giris_ms as i64)
        .ok_or_else(|| format!("geçersiz zaman: {giris_ms}"))?;
    let klasor = kok.join(zaman.format("%Y-%m").to_string());
    let yol = klasor.join("bolgeler.jsonl");
    let hata = |e: std::io::Error| format!("{}: {e}", yol.display());
    fs::create_dir_all(&klasor).map_err(hata)?;
    // Yarım kalmış son satırın üstüne yapışmasın.
    let mut metin = String::from(if yarim_satir_var(&yol) { "\n" } else { "" });
    metin += &serde_json::to_string(satir).map_err(|e| format!("{}: {e}", yol.display()))?;
    metin.push('\n');
    let mut dosya = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&yol)
        .map_err(hata)?;
    dosya.write_all(metin.as_bytes()).map_err(hata)
}

/// Dosya boş değilse ve son baytı satır sonu değilse (yarım satır) true; okunamazsa false.
fn yarim_satir_var(yol: &Path) -> bool {
    let Ok(mut f) = fs::File::open(yol) else {
        return false;
    };
    let mut son = [0_u8; 1];
    f.seek(SeekFrom::End(-1)).is_ok() && f.read_exact(&mut son).is_ok() && son[0] != b'\n'
}
