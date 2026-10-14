import type { Dil } from "../../../../ortak/dil";

interface Sozluk {
  savasBekleniyor: string; kilitle: string; kilidiAc: string; ayarlar: string;
  hedefYok: string; hp: string; kaldi: string; sira: string; sinif: string;
  oyuncu: string; dps: string; pay: string; ben: string; oldu: string;
  iyilestirme: string; hps: string; demoVerisi: string; surum: string;
  toplamDps: string; hasar: string;
}

export const metin: Record<Dil, Sozluk> = {
  tr: {
    savasBekleniyor: "Savaş bekleniyor", kilitle: "Kilitle", kilidiAc: "Kilidi aç", ayarlar: "Ayarlar",
    hedefYok: "Hedef yok", hp: "HP", kaldi: "{0} kaldı", sira: "sıra", sinif: "sınıf",
    oyuncu: "oyuncu", dps: "DPS", pay: "pay", ben: "BEN", oldu: "ÖLÜ",
    iyilestirme: "iyileştirme", hps: "HPS", demoVerisi: "Demo verisi", surum: "v0.0.7",
    toplamDps: "Toplam DPS", hasar: "Hasar",
  },
  en: {
    savasBekleniyor: "Waiting for combat", kilitle: "Lock", kilidiAc: "Unlock", ayarlar: "Settings",
    hedefYok: "No target", hp: "HP", kaldi: "{0} left", sira: "rank", sinif: "class",
    oyuncu: "player", dps: "DPS", pay: "share", ben: "YOU", oldu: "DEAD",
    iyilestirme: "healing", hps: "HPS", demoVerisi: "Demo data", surum: "v0.0.7",
    toplamDps: "Total DPS", hasar: "Damage",
  },
} as const;
