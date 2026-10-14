import type { Dil } from "../../../../ortak/dil";

export const metin = {
  tr: {
    canMeter: "CanMeter", savasBekleniyor: "Savaş bekleniyor", dps: "DPS", oyuncu: "Oyuncu", pay: "Pay",
    ben: "BEN", hps: "HPS", kaldi: "{0} kaldı", demoVerisi: "Demo verisi", surum: "v0.0.7",
    kilitle: "Kilitle", kilidiAc: "Kilidi aç", ayarlar: "Ayarlar", toplamDps: "Toplam DPS", hasar: "Hasar",
  },
  en: {
    canMeter: "CanMeter", savasBekleniyor: "Waiting for combat", dps: "DPS", oyuncu: "Player", pay: "Share",
    ben: "YOU", hps: "HPS", kaldi: "{0} left", demoVerisi: "Demo data", surum: "v0.0.7",
    kilitle: "Lock", kilidiAc: "Unlock", ayarlar: "Settings", toplamDps: "Total DPS", hasar: "Damage",
  },
} as const satisfies Record<Dil, Record<string, string>>;
