//! Npcap SDK bağlantısı (yalnız Windows hedefi).
//!
//! `pcap` 2.5 Windows'ta `wpcap.lib`'e bağlanır; SDK pakete gömülmez ve repoda durmaz
//! (GENEL §5: araçlar `<DerlemeAraclari>` altında, sürücü harfi koda yazılmaz).
//! Arama sırası: `NPCAP_SDK` ortam değişkeni, sonra bu reponun bulunduğu sürücüde
//! `\DerlemeAraclari\npcap-sdk`. Bulunamazsa uyarı basılır; link aşaması `wpcap.lib`'i
//! bulamayınca hata verir (derleme-kontrol `cargo check` etkilenmez).

use std::env;
use std::path::{Component, PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=NPCAP_SDK");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let mut adaylar: Vec<PathBuf> = Vec::new();
    if let Ok(p) = env::var("NPCAP_SDK") {
        adaylar.push(PathBuf::from(p));
    }
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    if let Some(Component::Prefix(onek)) = manifest.components().next() {
        let surucu = onek.as_os_str().to_string_lossy().into_owned();
        adaylar.push(PathBuf::from(format!(
            "{surucu}\\DerlemeAraclari\\npcap-sdk"
        )));
    }

    let alt = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("x86_64") => "Lib\\x64",
        Ok("aarch64") => "Lib\\ARM64",
        _ => "Lib",
    };
    for sdk in &adaylar {
        let dizin = sdk.join(alt);
        if dizin.join("wpcap.lib").is_file() {
            println!("cargo:rustc-link-search=native={}", dizin.display());
            return;
        }
    }
    println!(
        "cargo:warning=Npcap SDK bulunamadı (NPCAP_SDK ya da <sürücü>\\DerlemeAraclari\\npcap-sdk); \
         canlı yakalama bağlanamaz. Aranan: {}",
        adaylar
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(" | ")
    );
}
