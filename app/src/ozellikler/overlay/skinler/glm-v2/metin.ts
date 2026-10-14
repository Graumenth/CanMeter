import type { Dil } from "../../../../ortak/dil";

interface Sozluk {
  canMeter: string; demoVerisi: string; sure: string; hedef: string; hp: string;
  oyuncu: string; oyuncular: string; dps: string; pay: string; kaldi: string;
  sen: string; oldu: string; hps: string; savasBekleniyor: string; hedefYok: string;
  pasifDinleme: string; surum: string; kilitle: string; kilidiAc: string; ayarlar: string;
  toplamDps: string; hasar: string;
}

export const metin: Record<Dil, Sozluk> = {
  tr: {
    canMeter: "CanMeter", demoVerisi: "Demo verisi", sure: "Süre", hedef: "Hedef", hp: "HP",
    oyuncu: "Oyuncu", oyuncular: "{0} oyuncu", dps: "DPS", pay: "Pay", kaldi: "Tahmini kalan: {0}",
    sen: "Sen", oldu: "Öldü", hps: "HPS", savasBekleniyor: "Savaş bekleniyor", hedefYok: "Hedef yok",
    pasifDinleme: "Pasif dinleme: oyuna müdahale yok", surum: "v0.0.7", kilitle: "Kilitle", kilidiAc: "Kilidi aç", ayarlar: "Ayarlar",
    toplamDps: "Toplam DPS", hasar: "Hasar",
  },
  en: {
    canMeter: "CanMeter", demoVerisi: "Demo data", sure: "Time", hedef: "Target", hp: "HP",
    oyuncu: "Player", oyuncular: "{0} players", dps: "DPS", pay: "Share", kaldi: "Estimated left: {0}",
    sen: "You", oldu: "Dead", hps: "HPS", savasBekleniyor: "Waiting for combat", hedefYok: "No target",
    pasifDinleme: "Passive listening: no game interaction", surum: "v0.0.7", kilitle: "Lock", kilidiAc: "Unlock", ayarlar: "Settings",
    toplamDps: "Total DPS", hasar: "Damage",
  },
} as const;
