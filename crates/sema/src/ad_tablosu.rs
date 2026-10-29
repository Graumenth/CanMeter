//! Kimlik -> ad tablosu: TOML alt kümesi ayrıştırma (`kimlik = "Ad"`, `#` yorum, boş satır, `[...]` yok sayılır;
//! çift tırnak isteğe bağlı, tırnak içinde `\"` ve `\\` kaçışı, kapanıştan sonrası yok sayılır). Harita ve beceri
//! tabloları bunu sarar; ad tavanı tabloya göre değişir. Kullanıcı dosyası yolu `%APPDATA%\CanMeter\<dosya>`.
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AdTablosu {
    tablo: HashMap<u32, String>,
    tavan: usize,
}

/// Ad: satır sonları boşluğa, `trim`, `tavan` karakterde kırpılır.
pub fn temizle(ad: &str, tavan: usize) -> String {
    let duz = ad.replace(['\r', '\n'], " ");
    duz.trim().chars().take(tavan).collect()
}

/// `%APPDATA%\CanMeter\<dosya>`; APPDATA yoksa None.
pub fn kullanici_yolu(dosya: &str) -> Option<PathBuf> {
    let kok = std::env::var_os("APPDATA")?;
    Some(PathBuf::from(kok).join("CanMeter").join(dosya))
}

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

impl AdTablosu {
    /// `tavan`: ad en çok bu kadar karakter (`ekle` kırpar).
    pub fn new(tavan: usize) -> Self {
        Self {
            tablo: HashMap::new(),
            tavan,
        }
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

    pub fn ad(&self, kimlik: u32) -> Option<&str> {
        self.tablo.get(&kimlik).map(String::as_str)
    }

    /// Belleğe ekler (dosyaya yazmaz); ad `temizle` ile `trim` + tavan; boşsa etkisiz.
    pub fn ekle(&mut self, kimlik: u32, ad: &str) {
        let ad = temizle(ad, self.tavan);
        if !ad.is_empty() {
            self.tablo.insert(kimlik, ad);
        }
    }

    pub fn len(&self) -> usize {
        self.tablo.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tablo.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ayristirma() {
        let mut t = AdTablosu::new(60);
        t.ayristir("# yorum\n[baslik]\n\n1 = \"A \\\"B\\\" \\\\ C\"\n2 = Düz Ad\nx = y\n3 =\n");
        t.ayristir("4 = \"ilk\"\n4 = \"son\"\n");
        assert_eq!(t.len(), 3);
        assert_eq!(t.ad(1), Some("A \"B\" \\ C"));
        assert_eq!(t.ad(2), Some("Düz Ad"));
        assert_eq!(t.ad(3), None);
        assert_eq!(t.ad(4), Some("son"));
    }

    #[test]
    fn tavan_ve_yol() {
        let mut t = AdTablosu::new(3);
        t.ekle(1, "abcdef");
        t.ekle(2, " \n ");
        assert_eq!((t.ad(1), t.ad(2)), (Some("abc"), None));
        if std::env::var_os("APPDATA").is_some() {
            let yol = kullanici_yolu("x.toml").unwrap();
            assert!(yol.ends_with(std::path::Path::new("CanMeter").join("x.toml")));
        }
    }
}
