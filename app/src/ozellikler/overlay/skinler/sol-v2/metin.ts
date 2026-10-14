import type { Dil } from "../../../../ortak/dil";

interface Sozluk {
  canMeter: string; savas: string; hp: string; hedefAnalizi: string; kaldi: string;
  oyuncuSayisi: string; dps: string; pay: string; ben: string; oldu: string; hps: string;
  demoVerisi: string; surum: string; savasBekleniyor: string; kilitle: string;
  kilidiAc: string; ayarlar: string; toplamDps: string; hasar: string; hedefYok: string;
}

export const metin: Record<Dil, Sozluk> = {
  tr: {
    canMeter: "CanMeter", savas: "Savaş", hp: "HP", hedefAnalizi: "Hedef analizi", kaldi: "Tahmini {0} kaldı",
    oyuncuSayisi: "{0} oyuncu", dps: "DPS", pay: "Pay", ben: "BEN", oldu: "ÖLDÜ", hps: "HPS",
    demoVerisi: "Demo verisi", surum: "v0.0.7", savasBekleniyor: "Savaş bekleniyor", kilitle: "Kilitle",
    kilidiAc: "Kilidi aç", ayarlar: "Ayarlar", toplamDps: "Toplam DPS", hasar: "Hasar", hedefYok: "Hedef yok",
  },
  en: {
    canMeter: "CanMeter", savas: "Combat", hp: "HP", hedefAnalizi: "Target analysis", kaldi: "Estimated {0} left",
    oyuncuSayisi: "{0} players", dps: "DPS", pay: "Share", ben: "YOU", oldu: "DEAD", hps: "HPS",
    demoVerisi: "Demo data", surum: "v0.0.7", savasBekleniyor: "Waiting for combat", kilitle: "Lock",
    kilidiAc: "Unlock", ayarlar: "Settings", toplamDps: "Total DPS", hasar: "Damage", hedefYok: "No target",
  },
} as const;
