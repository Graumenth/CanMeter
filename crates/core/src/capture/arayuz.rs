//! Yakalama arayüzü (ağ adaptörü) seçimi.
//!
//! Npcap'in gördüğü adaptörleri listeler ve oyunun trafiğini taşıması en olası olanı seçer.
//! Strateji (faz 01 kararı):
//! 1. **Varsayılan rota:** makinenin internete çıkarken kullandığı yerel IPv4 adresi bulunur,
//!    o adresi taşıyan adaptör seçilir. Oyun sunucusu internette olduğundan trafik bu adaptörden
//!    geçer.
//! 2. **Yedek:** varsayılan rota bulunamazsa açık, loopback olmayan, link-local dışı IPv4'lü ilk
//!    adaptör (bağlı olmayan Wi-Fi/Bluetooth adaptörleri yalnız 169.254.x.x taşır, elenir).
//! 3. **Elle:** kullanıcı ad, açıklama parçası ya da liste sırası ile adaptör seçebilir
//!    (ayarlar / `--arayuz`). VPN ve ping düşürücü (loopback üstünden tünel) senaryoları için
//!    çoklu arayüz dinleme backlog'dadır.
//!
//! Hiçbir paket gönderilmez: varsayılan rota, bağlanmamış bir UDP soketine `connect` ile
//! öğrenilir. UDP `connect` yalnız çekirdekte rota araması yapıp yerel adresi bağlar; hat
//! üzerine datagram çıkmaz.

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};

use pcap::Device;

/// Npcap adaptörü, seçim için gereken özet bilgiyle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arayuz {
    /// Npcap aygıt adı (`\Device\NPF_{GUID}`); yakalama bu adla açılır.
    pub ad: String,
    /// İnsan okur açıklama (sürücü adı). Npcap vermezse boş.
    pub aciklama: String,
    /// Adaptöre atanmış IPv4 adresleri.
    pub ipv4: Vec<Ipv4Addr>,
    /// Loopback (Npcap Loopback Adapter) mı.
    pub loopback: bool,
    /// Adaptör açık ve çalışıyor mu (Npcap `UP` + `RUNNING` bayrakları).
    pub acik: bool,
}

impl Arayuz {
    /// Otomatik seçimde aday mı: açık, loopback değil, kullanılabilir bir IPv4'ü var.
    ///
    /// Npcap, kablosu çekili ya da bağlı olmayan adaptörleri de `UP`/`RUNNING` gösterir; onları
    /// ele veren şey yalnız link-local (169.254.0.0/16, APIPA) adres taşımalarıdır.
    fn aday_mi(&self) -> bool {
        self.acik && !self.loopback && self.ipv4.iter().any(|ip| !ip.is_link_local())
    }
}

impl fmt::Display for Arayuz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ipler: Vec<String> = self.ipv4.iter().map(ToString::to_string).collect();
        let ipler = if ipler.is_empty() {
            "ip yok".to_string()
        } else {
            ipler.join(", ")
        };
        let durum = match (self.acik, self.loopback) {
            (_, true) => "loopback",
            (true, false) => "açık",
            (false, false) => "kapalı",
        };
        let aciklama = if self.aciklama.is_empty() {
            self.ad.as_str()
        } else {
            self.aciklama.as_str()
        };
        write!(f, "{aciklama} [{durum}] {ipler}")
    }
}

/// Arayüz seçimi hataları.
#[derive(Debug)]
pub enum ArayuzHatasi {
    /// Npcap adaptör listesi alınamadı (sürücü yok, wpcap.dll yok, izin yok).
    Pcap(pcap::Error),
    /// Otomatik seçime uygun adaptör yok. Eldeki liste, kullanıcıya göstermek için.
    AdayYok(Vec<Arayuz>),
    /// Elle verilen seçim hiçbir adaptörle eşleşmedi.
    Bulunamadi(String),
}

impl fmt::Display for ArayuzHatasi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pcap(e) => write!(f, "Npcap adaptör listesi alınamadı: {e}"),
            Self::AdayYok(liste) => write!(
                f,
                "otomatik seçime uygun adaptör yok ({} adaptör görüldü)",
                liste.len()
            ),
            Self::Bulunamadi(secim) => write!(f, "adaptör bulunamadı: {secim:?}"),
        }
    }
}

impl std::error::Error for ArayuzHatasi {}

impl From<pcap::Error> for ArayuzHatasi {
    fn from(e: pcap::Error) -> Self {
        Self::Pcap(e)
    }
}

/// Npcap'in gördüğü adaptörleri, Npcap'in verdiği sırayla listeler.
pub fn listele() -> Result<Vec<Arayuz>, ArayuzHatasi> {
    Ok(Device::list()?.into_iter().map(Arayuz::from).collect())
}

impl From<Device> for Arayuz {
    fn from(d: Device) -> Self {
        let ipv4 = d
            .addresses
            .iter()
            .filter_map(|a| match a.addr {
                IpAddr::V4(ip) => Some(ip),
                IpAddr::V6(_) => None,
            })
            .collect();
        Self {
            ad: d.name,
            aciklama: d.desc.unwrap_or_default(),
            ipv4,
            loopback: d.flags.is_loopback(),
            acik: d.flags.is_up() && d.flags.is_running(),
        }
    }
}

/// Makinenin varsayılan rotasının yerel IPv4 adresi. Paket gönderilmez (modül notu).
///
/// Hedef adres yalnız rota aramasında kullanılır; erişilebilir olması gerekmez.
pub fn varsayilan_rota_ipv4() -> Option<Ipv4Addr> {
    let soket = UdpSocket::bind("0.0.0.0:0").ok()?;
    soket.connect("1.1.1.1:53").ok()?;
    match soket.local_addr().ok()?.ip() {
        IpAddr::V4(ip) if !ip.is_unspecified() => Some(ip),
        _ => None,
    }
}

/// Otomatik seçim: varsayılan rotayı taşıyan adaptör; yoksa ilk aday (modül notu, 1-2).
///
/// `rota_ip` dışarıdan verilir ki seçim mantığı sentetik listeyle test edilebilsin;
/// canlıda [`varsayilan_rota_ipv4`] ile beslenir.
pub fn otomatik_sec(liste: &[Arayuz], rota_ip: Option<Ipv4Addr>) -> Result<&Arayuz, ArayuzHatasi> {
    if let Some(ip) = rota_ip
        && let Some(a) = liste.iter().find(|a| a.aday_mi() && a.ipv4.contains(&ip))
    {
        return Ok(a);
    }
    liste
        .iter()
        .find(|a| a.aday_mi())
        .ok_or_else(|| ArayuzHatasi::AdayYok(liste.to_vec()))
}

/// Elle seçim. `secim` sırayla şunlardan biri olarak denenir: listedeki sıra numarası
/// (1'den başlar), tam aygıt adı, açıklamada geçen parça (büyük/küçük harf duyarsız).
pub fn elle_sec<'a>(liste: &'a [Arayuz], secim: &str) -> Result<&'a Arayuz, ArayuzHatasi> {
    let secim = secim.trim();
    if let Ok(sira) = secim.parse::<usize>()
        && sira >= 1
        && let Some(a) = liste.get(sira - 1)
    {
        return Ok(a);
    }
    if let Some(a) = liste.iter().find(|a| a.ad == secim) {
        return Ok(a);
    }
    let kucuk = secim.to_lowercase();
    liste
        .iter()
        .find(|a| !kucuk.is_empty() && a.aciklama.to_lowercase().contains(&kucuk))
        .ok_or_else(|| ArayuzHatasi::Bulunamadi(secim.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a(ad: &str, aciklama: &str, ipv4: &[&str], loopback: bool, acik: bool) -> Arayuz {
        Arayuz {
            ad: ad.to_string(),
            aciklama: aciklama.to_string(),
            ipv4: ipv4.iter().map(|s| s.parse().unwrap()).collect(),
            loopback,
            acik,
        }
    }

    fn ornek_liste() -> Vec<Arayuz> {
        vec![
            a(
                "\\Device\\NPF_Loopback",
                "Adapter for loopback traffic capture",
                &["127.0.0.1"],
                true,
                true,
            ),
            a(
                "\\Device\\NPF_{WIFI}",
                "Intel(R) Wi-Fi 6E",
                &[],
                false,
                false,
            ),
            a(
                "\\Device\\NPF_{BT}",
                "Bluetooth Device (PAN)",
                &["169.254.172.216"],
                false,
                true,
            ),
            a(
                "\\Device\\NPF_{VPN}",
                "TAP-Windows Adapter V9",
                &["10.8.0.2"],
                false,
                true,
            ),
            a(
                "\\Device\\NPF_{ETH}",
                "Realtek(R) PCI(e) Ethernet Controller",
                &["192.168.1.20"],
                false,
                true,
            ),
        ]
    }

    #[test]
    fn varsayilan_rota_adaptorunu_secer() {
        let liste = ornek_liste();
        let secilen = otomatik_sec(&liste, Some("192.168.1.20".parse().unwrap())).unwrap();
        assert_eq!(secilen.ad, "\\Device\\NPF_{ETH}");
    }

    #[test]
    fn rota_yoksa_ilk_acik_ipv4lu_loopback_olmayani_secer() {
        let liste = ornek_liste();
        let secilen = otomatik_sec(&liste, None).unwrap();
        assert_eq!(
            secilen.ad, "\\Device\\NPF_{VPN}",
            "loopback, kapalı Wi-Fi ve link-local Bluetooth atlanır"
        );
    }

    #[test]
    fn rota_ip_hicbir_adaptorde_yoksa_yedege_duser() {
        let liste = ornek_liste();
        let secilen = otomatik_sec(&liste, Some("172.16.0.9".parse().unwrap())).unwrap();
        assert_eq!(secilen.ad, "\\Device\\NPF_{VPN}");
    }

    #[test]
    fn aday_yoksa_hata_listeyi_tasir() {
        let liste = vec![
            a(
                "\\Device\\NPF_Loopback",
                "loopback",
                &["127.0.0.1"],
                true,
                true,
            ),
            a(
                "\\Device\\NPF_{X}",
                "kapalı",
                &["192.168.1.5"],
                false,
                false,
            ),
            a(
                "\\Device\\NPF_{Y}",
                "bağlı değil (yalnız link-local)",
                &["169.254.1.2"],
                false,
                true,
            ),
        ];
        match otomatik_sec(&liste, None) {
            Err(ArayuzHatasi::AdayYok(l)) => assert_eq!(l.len(), 3),
            diger => panic!("AdayYok bekleniyordu: {diger:?}"),
        }
    }

    #[test]
    fn elle_secim_sira_ad_ve_aciklama_parcasiyla() {
        let liste = ornek_liste();
        assert_eq!(elle_sec(&liste, "4").unwrap().ad, "\\Device\\NPF_{VPN}");
        assert_eq!(
            elle_sec(&liste, "\\Device\\NPF_{ETH}").unwrap().ad,
            "\\Device\\NPF_{ETH}"
        );
        assert_eq!(
            elle_sec(&liste, "realtek").unwrap().ad,
            "\\Device\\NPF_{ETH}"
        );
        assert!(matches!(
            elle_sec(&liste, "0"),
            Err(ArayuzHatasi::Bulunamadi(_))
        ));
        assert!(matches!(
            elle_sec(&liste, "99"),
            Err(ArayuzHatasi::Bulunamadi(_))
        ));
        assert!(matches!(
            elle_sec(&liste, "yok böyle"),
            Err(ArayuzHatasi::Bulunamadi(_))
        ));
        assert!(matches!(
            elle_sec(&liste, ""),
            Err(ArayuzHatasi::Bulunamadi(_))
        ));
    }
}
