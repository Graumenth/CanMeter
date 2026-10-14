//! Npcap'in gördüğü adaptörleri listeler ve otomatik seçimin hangisine düştüğünü gösterir.
//!
//! `cargo run -p canmeter-core --example arayuzler`

use canmeter_core::capture::arayuz;

fn main() {
    let liste = match arayuz::listele() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("HATA: {e}");
            std::process::exit(1);
        }
    };
    let rota = arayuz::varsayilan_rota_ipv4();
    let secilen = arayuz::otomatik_sec(&liste, rota)
        .ok()
        .map(|a| a.ad.clone());

    println!(
        "Varsayılan rota IPv4: {}",
        rota.map_or("bulunamadı".to_string(), |ip| ip.to_string())
    );
    println!("{} adaptör:", liste.len());
    for (i, a) in liste.iter().enumerate() {
        let isaret = if secilen.as_deref() == Some(a.ad.as_str()) {
            "=>"
        } else {
            "  "
        };
        println!("{isaret} {:>2}. {a}", i + 1);
        println!("       {}", a.ad);
    }
    if secilen.is_none() {
        eprintln!("Otomatik seçim başarısız; --arayuz ile elle seçin.");
        std::process::exit(2);
    }
}
