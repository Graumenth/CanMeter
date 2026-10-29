//! Kullanıcı harita dosyası: yol çözümü ve satır ekleme. Tablo ayrıştırma `ad_tablosu.rs` içindedir.
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use super::AD_TAVANI;

const BASLIK: &str = "# CanMeter kullanıcı harita adları: kimlik = \"Ad\"\n";

/// `%APPDATA%\CanMeter\haritalar.toml`; APPDATA yoksa None.
pub fn kullanici_yolu() -> Option<PathBuf> {
    crate::ad_tablosu::kullanici_yolu("haritalar.toml")
}

/// Kullanıcı dosyasına bir satır ekler (klasörü oluşturur; yeni dosyaya önce `#` başlık yorumu). Ad `trim`,
/// satır sonları boşluğa, AD_TAVANI karakterde kırpılır; boşsa `Err`. Değer tırnaklı, `"` ve `\` kaçışlı yazılır.
/// Son satır sonu eksik bir dosyaya önce satır sonu eklenir. Belleğe yazmaz; çağıran `ekle` ile tamamlar.
pub fn kaydet(yol: &Path, harita: u32, ad: &str) -> Result<(), String> {
    let ad = crate::ad_tablosu::temizle(ad, AD_TAVANI);
    if ad.is_empty() {
        return Err("ad boş".to_string());
    }
    let es = |e: std::io::Error| e.to_string();
    if let Some(klasor) = yol.parent() {
        std::fs::create_dir_all(klasor).map_err(es)?;
    }
    let baslik = if yol.exists() { "" } else { BASLIK };
    let on = if satir_basi_gerek(yol) { "\n" } else { "" };
    let kacisli = ad.replace('\\', "\\\\").replace('"', "\\\"");
    let satir = format!("{on}{baslik}{harita} = \"{kacisli}\"\n");
    let acik = OpenOptions::new().create(true).append(true).open(yol);
    acik.and_then(|mut d| d.write_all(satir.as_bytes()))
        .map_err(es)
}

/// Dosya boş değilse ve son baytı satır sonu değilse `true`. Okunamazsa (boş dosya dahil) `false`; ekleme yine yapılır.
fn satir_basi_gerek(yol: &Path) -> bool {
    let Ok(mut d) = File::open(yol) else {
        return false;
    };
    if d.seek(SeekFrom::End(-1)).is_err() {
        return false;
    }
    let mut son = [0u8; 1];
    d.read_exact(&mut son).is_ok() && son[0] != b'\n'
}
