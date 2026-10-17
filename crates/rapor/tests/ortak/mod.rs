use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

static SAYAC: AtomicU64 = AtomicU64::new(0);

pub struct Gecici(pub PathBuf);

impl Gecici {
    pub fn yeni() -> Self {
        let yol = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(".gecici-testler")
            .join(format!(
                "{}-{}",
                std::process::id(),
                SAYAC.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(&yol).unwrap();
        Self(yol)
    }
}

impl Drop for Gecici {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

pub fn ornekler() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("ornek")
}
