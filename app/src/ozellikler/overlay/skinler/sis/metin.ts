import type { Dil } from "../../../../ortak/dil";

export const metin: Record<Dil, Record<string, string>> = {
  tr: {
    canMeter: "CanMeter", savasBekleniyor: "Savaş bekleniyor", dps: "DPS", hedef: "Hedef",
    hedefYok: "Hedef yok", hp: "HP", kaldi: "{0} kaldı", sira: "Sıra", sinif: "Sınıf",
    oyuncu: "Oyuncu", pay: "Pay", ben: "BEN", olu: "ÖLÜ", hps: "HPS",
    demoVerisi: "Demo verisi", surum: "v0.0.7", kilitle: "Kilitle", kilidiAc: "Kilidi aç",
    ayarlar: "Ayarlar", toplamDps: "Toplam DPS", hasar: "Hasar", savasSuresi: "Savaş süresi",
    oyuncuHasari: "Oyuncu hasar sıralaması", hedefSagligi: "Hedef sağlığı",
  },
  en: {
    canMeter: "CanMeter", savasBekleniyor: "Waiting for combat", dps: "DPS", hedef: "Target",
    hedefYok: "No target", hp: "HP", kaldi: "{0} left", sira: "Rank", sinif: "Class",
    oyuncu: "Player", pay: "Share", ben: "YOU", olu: "DEAD", hps: "HPS",
    demoVerisi: "Demo data", surum: "v0.0.7", kilitle: "Lock", kilidiAc: "Unlock",
    ayarlar: "Settings", toplamDps: "Total DPS", hasar: "Damage", savasSuresi: "Combat time",
    oyuncuHasari: "Player damage ranking", hedefSagligi: "Target health",
  },
} as const;
