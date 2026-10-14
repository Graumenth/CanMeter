import type { Dil } from "../../../../ortak/dil";

interface Sozluk {
  canMeter: string; savasBekleniyor: string; sen: string; oldu: string; hps: string;
  hedefYok: string; sure: string; kalan: string;
  demoVerisi: string; kilitle: string; kilidiAc: string; ayarlar: string;
  toplamDps: string; hasar: string; surum: string;
}

export const metin: Record<Dil, Sozluk> = {
  tr: {
    canMeter: "CanMeter", savasBekleniyor: "Savaş bekleniyor", sen: "SEN", oldu: "ÖLDÜ", hps: "HPS",
    hedefYok: "Hedef yok", sure: "Süre", kalan: "kalan", demoVerisi: "Demo verisi",
    kilitle: "Kilitle", kilidiAc: "Kilidi aç",
    ayarlar: "Ayarlar", toplamDps: "Toplam DPS", hasar: "Hasar", surum: "CanMeter v0.0.7",
  },
  en: {
    canMeter: "CanMeter", savasBekleniyor: "Waiting for combat", sen: "YOU", oldu: "DEAD", hps: "HPS",
    hedefYok: "No target", sure: "Time", kalan: "left", demoVerisi: "Demo data",
    kilitle: "Lock", kilidiAc: "Unlock",
    ayarlar: "Settings", toplamDps: "Total DPS", hasar: "Damage", surum: "CanMeter v0.0.7",
  },
} as const;
