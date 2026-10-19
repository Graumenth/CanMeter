use canmeter_rapor::komut::cozumle;
use std::{ffi::OsString, fs, path::PathBuf, process::Command};

fn sozler(sozler: &[&str]) -> Vec<OsString> {
    sozler.iter().map(OsString::from).collect()
}

#[test]
fn yollar_ac_ve_eksik_arguman() {
    let secenek = cozumle(sozler(&[
        "--girdi",
        "girdi klasörü",
        "--cikti",
        "çıktı",
        "--ac",
    ]))
    .unwrap();
    assert_eq!(secenek.girdi, PathBuf::from("girdi klasörü"));
    assert_eq!(secenek.cikti, PathBuf::from("çıktı"));
    assert!(secenek.ac);
    assert!(secenek.girdi_verildi);
    for arguman in [vec!["--girdi"], vec!["--cikti", "--ac"], vec!["--yanlis"]] {
        assert!(cozumle(sozler(&arguman)).is_err());
    }
}

#[test]
fn hata_stderr_ve_cikis_bir() {
    let sonuc = Command::new(env!("CARGO_BIN_EXE_canmeter-rapor"))
        .arg("--yanlis")
        .output()
        .unwrap();
    assert_eq!(sonuc.status.code(), Some(1));
    assert!(sonuc.stdout.is_empty());
    assert!(
        String::from_utf8(sonuc.stderr)
            .unwrap()
            .contains("Hata: Bilinmeyen seçenek")
    );
}

#[test]
fn varsayilan_yollar() {
    let sonuc = Command::new(env!("CARGO_BIN_EXE_canmeter-rapor"))
        .env_remove("LOCALAPPDATA")
        .output()
        .unwrap();
    assert_eq!(sonuc.status.code(), Some(1));
    assert!(
        String::from_utf8(sonuc.stderr)
            .unwrap()
            .contains("LOCALAPPDATA tanımlı değil")
    );
    if let Some(yerel) = std::env::var_os("LOCALAPPDATA") {
        let secenek = cozumle(Vec::<OsString>::new()).unwrap();
        assert_eq!(
            secenek.girdi,
            PathBuf::from(&yerel).join("CanMeter/encounters")
        );
        assert_eq!(secenek.cikti, PathBuf::from(yerel).join("CanMeter/rapor"));
        assert!(!secenek.ac);
        assert!(!secenek.girdi_verildi);
    }
}

fn gecici_kok(ad: &str) -> PathBuf {
    let kok =
        PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("{ad}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&kok);
    fs::create_dir_all(&kok).unwrap();
    kok
}

#[test]
fn varsayilan_klasor_yoksa_bos_rapor() {
    let kok = gecici_kok("varsayilan");
    let sonuc = Command::new(env!("CARGO_BIN_EXE_canmeter-rapor"))
        .env("LOCALAPPDATA", &kok)
        .output()
        .unwrap();
    assert_eq!(sonuc.status.code(), Some(0));
    assert!(
        String::from_utf8(sonuc.stdout)
            .unwrap()
            .starts_with("0 savaş raporu yazıldı")
    );
    assert!(
        String::from_utf8(sonuc.stderr)
            .unwrap()
            .contains("henüz yok")
    );
    let dizin = fs::read_to_string(kok.join("CanMeter/rapor/index.html")).unwrap();
    assert!(dizin.contains("Henüz kayıtlı savaş yok"));
    fs::remove_dir_all(&kok).unwrap();
}

#[test]
fn acik_girdi_yoksa_hata() {
    let kok = gecici_kok("acik");
    let sonuc = Command::new(env!("CARGO_BIN_EXE_canmeter-rapor"))
        .arg("--girdi")
        .arg(kok.join("yok"))
        .arg("--cikti")
        .arg(kok.join("rapor"))
        .output()
        .unwrap();
    assert_eq!(sonuc.status.code(), Some(1));
    assert!(
        String::from_utf8(sonuc.stderr)
            .unwrap()
            .contains("klasörü okunamadı")
    );
    fs::remove_dir_all(&kok).unwrap();
}
