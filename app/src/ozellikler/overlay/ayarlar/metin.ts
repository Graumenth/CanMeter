import type { Dil } from "../../../ortak/dil";

interface Sozluk {
  ayarlar: string;
  kapat: string;
  dil: string;
  gorunum: string;
  seffaflik: string;
  zeminRengi: string;
  varsayilan: string;
  toplamGostergesi: string;
  kisayollar: string;
  gorunumKisayol: string;
  dilKisayol: string;
}

export const metin: Record<Dil, Sozluk> = {
  tr: {
    ayarlar: "Ayarlar", kapat: "Kapat", dil: "Dil", gorunum: "Görünüm",
    seffaflik: "Şeffaflık", zeminRengi: "Zemin rengi", varsayilan: "Varsayılan",
    toplamGostergesi: "Toplam göstergesi", kisayollar: "Kısayollar",
    gorunumKisayol: "Ctrl+Shift+S görünüm", dilKisayol: "Ctrl+Shift+L dil",
  },
  en: {
    ayarlar: "Settings", kapat: "Close", dil: "Language", gorunum: "Appearance",
    seffaflik: "Opacity", zeminRengi: "Background color", varsayilan: "Default",
    toplamGostergesi: "Totals display", kisayollar: "Shortcuts",
    gorunumKisayol: "Ctrl+Shift+S appearance", dilKisayol: "Ctrl+Shift+L language",
  },
};
