//! Mesaj alanlarını sınır denetimli ve ayırmasız okuyan küçük yardımcı.

use crate::protocol::varint;

pub(super) struct Okuyucu<'a> {
    veri: &'a [u8],
    konum: usize,
}

impl<'a> Okuyucu<'a> {
    pub(super) fn yeni(veri: &'a [u8]) -> Self {
        Self { veri, konum: 0 }
    }

    pub(super) fn u8(&mut self) -> Option<u8> {
        let deger = *self.veri.get(self.konum)?;
        self.konum += 1;
        Some(deger)
    }

    pub(super) fn u32(&mut self) -> Option<u32> {
        let son = self.konum.checked_add(4)?;
        let alan: [u8; 4] = self.veri.get(self.konum..son)?.try_into().ok()?;
        self.konum = son;
        Some(u32::from_le_bytes(alan))
    }

    pub(super) fn varint(&mut self) -> Option<u32> {
        let kalan = self.veri.get(self.konum..)?;
        let (deger, boyut) = varint::oku(kalan).ok()?;
        self.konum = self.konum.checked_add(boyut)?;
        Some(deger)
    }

    pub(super) fn atla(&mut self, boyut: usize) -> Option<()> {
        let son = self.konum.checked_add(boyut)?;
        self.veri.get(self.konum..son)?;
        self.konum = son;
        Some(())
    }

    pub(super) fn bitti(&self) -> bool {
        self.konum == self.veri.len()
    }

    /// Okunmamış bayt sayısı.
    pub(super) fn kalan(&self) -> usize {
        self.veri.len().saturating_sub(self.konum)
    }
}
