use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, Instant};

use canmeter_core::capture::arayuz::{self, Arayuz};
use chrono::Local;

const KULLANIM: &str = "Kullanım: kaydet [--arayuz <sıra|ad|açıklama parçası>] [--sure <saniye>] [--cikti <yol>] [--filtre <bpf>] [--liste]
Varsayılan: otomatik adaptör, filtre tcp, çıktı kayitlar/<tarih>.pcap. Akış özeti yalnız IPv4/TCP sayar; diğer paketler yine dosyaya yazılır.";

struct Ayarlar {
    arayuz_secimi: Option<String>,
    sure: Option<Duration>,
    cikti_yolu: Option<PathBuf>,
    filtre: String,
    liste: bool,
}

#[derive(Default)]
struct Akis {
    gelen: u64,
    giden: u64,
    paket: u64,
}

fn main() -> ExitCode {
    match calistir() {
        Ok(()) => ExitCode::SUCCESS,
        Err((kod, mesaj)) => {
            if kod == 0 {
                println!("{KULLANIM}");
                return ExitCode::SUCCESS;
            }
            if !mesaj.is_empty() {
                eprintln!("{mesaj}");
            }
            if kod == 2 {
                eprintln!("{KULLANIM}");
            }
            ExitCode::from(kod)
        }
    }
}

fn calistir() -> Result<(), (u8, String)> {
    let ayarlar = ayarlari_oku()?;
    let liste = arayuz::listele().map_err(hata)?;
    let otomatik = if ayarlar.liste || ayarlar.arayuz_secimi.is_none() {
        arayuz::otomatik_sec(&liste, arayuz::varsayilan_rota_ipv4()).ok()
    } else {
        None
    };

    if ayarlar.liste {
        for (sira, a) in liste.iter().enumerate() {
            let isaret = otomatik.is_some_and(|secili| secili.ad == a.ad);
            println!(
                "{}{}: {} | ad: {}",
                if isaret { "=> " } else { "   " },
                sira + 1,
                a,
                a.ad
            );
        }
        return Ok(());
    }

    let secilen = match ayarlar.arayuz_secimi.as_deref() {
        Some(secim) => arayuz::elle_sec(&liste, secim).map_err(hata)?,
        None => {
            otomatik.ok_or_else(|| (1, "Otomatik adaptör seçimi başarısız oldu".to_string()))?
        }
    };
    let cikti_yolu = cikti_yolunu_hazirla(ayarlar.cikti_yolu)?;
    kaydet(secilen, &cikti_yolu, &ayarlar.filtre, ayarlar.sure)
}

fn ayarlari_oku() -> Result<Ayarlar, (u8, String)> {
    let mut arayuz_secimi = None;
    let mut sure = None;
    let mut cikti_yolu = None;
    let mut filtre = String::from("tcp");
    let mut liste = false;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--liste" => liste = true,
            "-h" | "--yardim" => return Err((0, String::new())),
            "--arayuz" => arayuz_secimi = Some(deger_al(&mut args, "--arayuz")?),
            "--sure" => {
                let deger = deger_al(&mut args, "--sure")?;
                let saniye = deger
                    .parse::<u64>()
                    .map_err(|_| (2, format!("Geçersiz süre: {deger}")))?;
                sure = Some(Duration::from_secs(saniye));
            }
            "--cikti" => cikti_yolu = Some(PathBuf::from(deger_al(&mut args, "--cikti")?)),
            "--filtre" => filtre = deger_al(&mut args, "--filtre")?,
            _ => return Err((2, format!("Bilinmeyen argüman: {arg}"))),
        }
    }
    Ok(Ayarlar {
        arayuz_secimi,
        sure,
        cikti_yolu,
        filtre,
        liste,
    })
}

fn deger_al(
    args: &mut impl Iterator<Item = String>,
    secenek: &str,
) -> Result<String, (u8, String)> {
    args.next()
        .ok_or_else(|| (2, format!("{secenek} için değer gerekli")))
}

fn hata(hata: impl std::fmt::Display) -> (u8, String) {
    (1, hata.to_string())
}

fn cikti_yolunu_hazirla(cikti: Option<PathBuf>) -> Result<PathBuf, (u8, String)> {
    if let Some(yol) = cikti {
        return Ok(yol);
    }
    // Çalışma dizinine değil workspace köküne göre: crates/core içinden koşulsa da kayitlar/ tek yerde.
    let kok = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../kayitlar");
    std::fs::create_dir_all(&kok).map_err(hata)?;
    let damga = Local::now().format("%Y-%m-%d_%H%M");
    Ok(kok.join(format!("{damga}.pcap")))
}

fn kaydet(
    arayuz: &Arayuz,
    cikti_yolu: &Path,
    filtre: &str,
    sure: Option<Duration>,
) -> Result<(), (u8, String)> {
    let mut cap = pcap::Capture::from_device(arayuz.ad.as_str())
        .map_err(hata)?
        .promisc(false)
        .snaplen(65535)
        .immediate_mode(true)
        .timeout(500)
        .open()
        .map_err(hata)?;
    cap.filter(filtre, true).map_err(hata)?;
    let mut savefile = cap.savefile(cikti_yolu).map_err(hata)?;
    let durdur = Arc::new(AtomicBool::new(false));
    let durdur_kopya = Arc::clone(&durdur);
    ctrlc::set_handler(move || durdur_kopya.store(true, Ordering::SeqCst)).map_err(hata)?;

    println!("Adaptör: {arayuz}");
    println!("Filtre: {filtre}");
    println!("Çıktı: {}", cikti_yolu.display());
    match sure {
        Some(saniye) => println!(
            "{} saniye kaydedilecek. Ctrl+C ile durdurun.",
            saniye.as_secs()
        ),
        None => println!("Ctrl+C ile durdurun."),
    }

    let baslangic = Instant::now();
    let mut son_flush = baslangic;
    let mut son_ozet = baslangic;
    let mut toplam_paket = 0_u64;
    let mut sayilmayan = 0_u64;
    // Akış anahtarı uzak IPv4 adresi ve uzak TCP portudur.
    let mut akislar: HashMap<(Ipv4Addr, u16), Akis> = HashMap::new();
    let mut hata_oldu = false;

    loop {
        if durdur.load(Ordering::SeqCst) || sure.is_some_and(|s| baslangic.elapsed() >= s) {
            break;
        }
        match cap.next_packet() {
            Ok(paket) => {
                // Paket önce dosyaya yazılır; çözümlenemeyen paketler de kaybolmaz.
                savefile.write(&paket);
                toplam_paket += 1;
                match akis_bilgisi(paket.data, &arayuz.ipv4) {
                    Some((uzak_ip, uzak_port, gelen)) => {
                        let akis = akislar.entry((uzak_ip, uzak_port)).or_default();
                        akis.paket += 1;
                        if gelen {
                            akis.gelen += u64::from(paket.header.len);
                        } else {
                            akis.giden += u64::from(paket.header.len);
                        }
                    }
                    None => sayilmayan += 1,
                }
            }
            Err(pcap::Error::TimeoutExpired) => {}
            Err(hata) => {
                eprintln!("Yakalama hatası: {hata}");
                hata_oldu = true;
                break;
            }
        }
        if son_flush.elapsed() >= Duration::from_secs(5) {
            savefile.flush().map_err(hata)?;
            son_flush = Instant::now();
        }
        if son_ozet.elapsed() >= Duration::from_secs(10) {
            ozet_yazdir(toplam_paket, cikti_yolu, sayilmayan, &akislar, 5);
            son_ozet = Instant::now();
        }
    }

    savefile.flush().map_err(hata)?;
    ozet_yazdir(toplam_paket, cikti_yolu, sayilmayan, &akislar, 10);
    oyun_adayi_yazdir(&akislar);
    drop(savefile);
    let boyut = std::fs::metadata(cikti_yolu).map_err(hata)?.len();
    println!("Dosya boyutu: {boyut} bayt");
    if hata_oldu {
        Err((1, "Yakalama sırasında hata oluştu".to_string()))
    } else {
        Ok(())
    }
}

fn akis_bilgisi(veri: &[u8], yerel_ipler: &[Ipv4Addr]) -> Option<(Ipv4Addr, u16, bool)> {
    if veri.len() < 14 || u16::from_be_bytes([veri[12], veri[13]]) != 0x0800 {
        return None;
    }
    let ip_baslangic = 14;
    let ip_uzunluk = usize::from(veri[ip_baslangic] & 0x0f) * 4;
    if veri[ip_baslangic] >> 4 != 4 || ip_uzunluk < 20 || veri.len() < ip_baslangic + ip_uzunluk {
        return None;
    }
    if veri[ip_baslangic + 9] != 6 {
        return None;
    }
    let kaynak = Ipv4Addr::new(veri[26], veri[27], veri[28], veri[29]);
    let hedef = Ipv4Addr::new(veri[30], veri[31], veri[32], veri[33]);
    let tcp = ip_baslangic + ip_uzunluk;
    if veri.len() < tcp + 20 {
        return None;
    }
    let tcp_uzunluk = usize::from(veri[tcp + 12] >> 4) * 4;
    if tcp_uzunluk < 20 || veri.len() < tcp + tcp_uzunluk {
        return None;
    }
    let kaynak_port = u16::from_be_bytes([veri[tcp], veri[tcp + 1]]);
    let hedef_port = u16::from_be_bytes([veri[tcp + 2], veri[tcp + 3]]);
    let kaynak_yerel = yerel_ipler.contains(&kaynak);
    let hedef_yerel = yerel_ipler.contains(&hedef);
    // Dönüş: (uzak ip, uzak port, gelen mi). Yerelden çıkan paket "giden", yerele gelen "gelen".
    if kaynak_yerel && !hedef_yerel {
        Some((hedef, hedef_port, false))
    } else if hedef_yerel && !kaynak_yerel {
        Some((kaynak, kaynak_port, true))
    } else if !kaynak_yerel && !hedef_yerel {
        // İki uç da yerel değil (ör. elle seçilen adaptörün IP'si listede yok): kaynak uzak sayılır.
        Some((kaynak, kaynak_port, true))
    } else {
        None
    }
}

fn ozet_yazdir(
    toplam: u64,
    yol: &Path,
    sayilmayan: u64,
    akislar: &HashMap<(Ipv4Addr, u16), Akis>,
    adet: usize,
) {
    let mut sirali: Vec<_> = akislar.iter().collect();
    sirali.sort_unstable_by_key(|a| std::cmp::Reverse(a.1.gelen + a.1.giden));
    println!(
        "Özet: toplam paket={toplam}  dosya={}  sayılmayan={sayilmayan}",
        yol.display()
    );
    for ((ip, port), akis) in sirali.into_iter().take(adet) {
        println!(
            "{ip}:{port}  gelen={}  giden={}  paket={}",
            akis.gelen, akis.giden, akis.paket
        );
    }
}

fn oyun_adayi_yazdir(akislar: &HashMap<(Ipv4Addr, u16), Akis>) {
    let aday = akislar
        .iter()
        .filter(|((_, port), _)| *port != 80 && *port != 443)
        .max_by_key(|(_, akis)| akis.gelen);
    match aday {
        Some(((ip, port), _)) => println!("Oyun sunucusu adayı: {ip}:{port}"),
        None => println!("Oyun sunucusu adayı: aday yok"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ethernet + IPv4 (20 bayt) + TCP (20 bayt) başlığı, yük yok.
    fn tcp_cercevesi(
        kaynak: [u8; 4],
        kaynak_port: u16,
        hedef: [u8; 4],
        hedef_port: u16,
    ) -> Vec<u8> {
        let mut v = vec![0u8; 14];
        v[12] = 0x08;
        v[13] = 0x00;
        let mut ip = vec![0u8; 20];
        ip[0] = 0x45;
        ip[9] = 6;
        ip[12..16].copy_from_slice(&kaynak);
        ip[16..20].copy_from_slice(&hedef);
        let mut tcp = vec![0u8; 20];
        tcp[0..2].copy_from_slice(&kaynak_port.to_be_bytes());
        tcp[2..4].copy_from_slice(&hedef_port.to_be_bytes());
        tcp[12] = 0x50;
        v.extend(ip);
        v.extend(tcp);
        v
    }

    #[test]
    fn yerelden_cikan_paket_gidendir() {
        let yerel = [Ipv4Addr::new(192, 168, 1, 105)];
        let cerceve = tcp_cercevesi([192, 168, 1, 105], 50000, [203, 0, 113, 7], 13328);
        assert_eq!(
            akis_bilgisi(&cerceve, &yerel),
            Some((Ipv4Addr::new(203, 0, 113, 7), 13328, false))
        );
    }

    #[test]
    fn yerele_gelen_paket_gelendir() {
        let yerel = [Ipv4Addr::new(192, 168, 1, 105)];
        let cerceve = tcp_cercevesi([203, 0, 113, 7], 13328, [192, 168, 1, 105], 50000);
        assert_eq!(
            akis_bilgisi(&cerceve, &yerel),
            Some((Ipv4Addr::new(203, 0, 113, 7), 13328, true))
        );
    }

    #[test]
    fn ipv4_tcp_disi_ve_kisa_paket_sayilmaz() {
        let yerel = [Ipv4Addr::new(192, 168, 1, 105)];
        let mut udp = tcp_cercevesi([203, 0, 113, 7], 53, [192, 168, 1, 105], 50000);
        udp[14 + 9] = 17;
        assert_eq!(akis_bilgisi(&udp, &yerel), None);
        let kisa = &tcp_cercevesi([203, 0, 113, 7], 1, [192, 168, 1, 105], 2)[..40];
        assert_eq!(akis_bilgisi(kisa, &yerel), None);
        let mut ipv6 = tcp_cercevesi([1, 1, 1, 1], 1, [192, 168, 1, 105], 2);
        ipv6[12] = 0x86;
        ipv6[13] = 0xdd;
        assert_eq!(akis_bilgisi(&ipv6, &yerel), None);
    }
}
