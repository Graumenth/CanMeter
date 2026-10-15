//! Ham Ethernet çerçevesinden IPv4/TCP segmenti çıkarır.
use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment<'a> {
    pub kaynak: SocketAddrV4,
    pub hedef: SocketAddrV4,
    /// TCP sıra numarası (yükün ilk baytı).
    pub seq: u32,
    pub yuk: &'a [u8],
}

/// Ethernet II (isteğe bağlı 802.1Q etiketi) -> IPv4 -> TCP.
pub fn ayristir(ethernet: &[u8]) -> Option<Segment<'_>> {
    let mut ip = 14;
    let mut tur = ethernet.get(12..14)?;
    if tur == [0x81, 0x00] {
        tur = ethernet.get(16..18)?;
        ip += 4;
    }
    if tur != [0x08, 0x00] || ethernet.get(ip)? >> 4 != 4 {
        return None;
    }
    let ip_len = usize::from(ethernet[ip] & 0x0f) * 4;
    if ip_len < 20 || ethernet.len() < ip + ip_len || ethernet.get(ip + 9) != Some(&6) {
        return None;
    }
    let tcp = ip + ip_len;
    let tcp_len = usize::from(*ethernet.get(tcp + 12)? >> 4) * 4;
    if tcp_len < 20 || ethernet.len() < tcp + tcp_len {
        return None;
    }
    let toplam = usize::from(u16::from_be_bytes(
        ethernet.get(ip + 2..ip + 4)?.try_into().ok()?,
    ));
    let yuk_len = toplam.checked_sub(ip_len + tcp_len)?;
    let son = (tcp + tcp_len + yuk_len).min(ethernet.len());
    let kaynak_ip = Ipv4Addr::new(
        ethernet[ip + 12],
        ethernet[ip + 13],
        ethernet[ip + 14],
        ethernet[ip + 15],
    );
    let hedef_ip = Ipv4Addr::new(
        ethernet[ip + 16],
        ethernet[ip + 17],
        ethernet[ip + 18],
        ethernet[ip + 19],
    );
    Some(Segment {
        kaynak: SocketAddrV4::new(
            kaynak_ip,
            u16::from_be_bytes(ethernet[tcp..tcp + 2].try_into().ok()?),
        ),
        hedef: SocketAddrV4::new(
            hedef_ip,
            u16::from_be_bytes(ethernet[tcp + 2..tcp + 4].try_into().ok()?),
        ),
        seq: u32::from_be_bytes(ethernet[tcp + 4..tcp + 8].try_into().ok()?),
        yuk: &ethernet[tcp + tcp_len..son],
    })
}
#[cfg(test)]
mod testler {
    use super::*;
    fn cerceve() -> Vec<u8> {
        let mut v = vec![0; 54];
        v[12..14].copy_from_slice(&[8, 0]);
        v[14] = 0x45;
        v[23] = 6;
        v[26..30].copy_from_slice(&[1, 2, 3, 4]);
        v[30..34].copy_from_slice(&[5, 6, 7, 8]);
        v[34..36].copy_from_slice(&10u16.to_be_bytes());
        v[36..38].copy_from_slice(&20u16.to_be_bytes());
        v[38..42].copy_from_slice(&99u32.to_be_bytes());
        v[46] = 0x50;
        v
    }
    fn yuklu() -> Vec<u8> {
        let mut v = cerceve();
        v[16..18].copy_from_slice(&43u16.to_be_bytes());
        v.extend(b"abc");
        v
    }
    #[test]
    fn tcp_yuku() {
        let v = yuklu();
        let s = ayristir(&v).unwrap();
        assert_eq!(s.yuk, b"abc");
        assert_eq!(s.seq, 99);
    }
    #[test]
    fn vlan() {
        let mut v = yuklu();
        v.splice(12..14, [0x81, 0, 0, 1, 8, 0]);
        assert_eq!(ayristir(&v).unwrap().yuk, b"abc");
    }
    #[test]
    fn dolgu() {
        let mut v = yuklu();
        v.extend([0; 20]);
        assert_eq!(ayristir(&v).unwrap().yuk, b"abc");
    }
    #[test]
    fn udp() {
        let mut v = yuklu();
        v[23] = 17;
        assert!(ayristir(&v).is_none());
    }
    #[test]
    fn ipv6() {
        let mut v = yuklu();
        v[12..14].copy_from_slice(&[0x86, 0xdd]);
        assert!(ayristir(&v).is_none());
    }
    #[test]
    fn kesik() {
        assert!(ayristir(&cerceve()[..33]).is_none());
    }
}
