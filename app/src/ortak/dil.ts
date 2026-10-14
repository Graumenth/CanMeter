import { useEffect, useSyncExternalStore } from "react";

export type Dil = "tr" | "en";
export const diller: Dil[] = ["tr", "en"];

const baslangic = (): Dil => {
  try {
    // Geliştirme kolaylığı: ?dil=en ilk seçimi belirler (kaydedilmez).
    const param = new URLSearchParams(location.search).get("dil");
    if (param === "tr" || param === "en") return param;
    const kayit = localStorage.getItem("dil");
    if (kayit === "tr" || kayit === "en") return kayit;
  } catch { /* localStorage kullanilamazsa tarayiciyi kullan. */ }
  // Ürün Türkçe arayüzlü (README); İngilizce ayarlardan seçilir.
  return "tr";
};

let mevcut: Dil = baslangic();
const aboneler = new Set<() => void>();
const abone = (dinleyici: () => void) => { aboneler.add(dinleyici); return () => { aboneler.delete(dinleyici); }; };
const anlik = () => mevcut;
let dinleyen = 0;
const tus = (olay: KeyboardEvent) => {
  if (olay.ctrlKey && olay.shiftKey && olay.key.toLowerCase() === "l") {
    olay.preventDefault();
    dilDegistir();
  }
};

export function useDil(): Dil {
  useEffect(() => {
    if (dinleyen++ === 0) window.addEventListener("keydown", tus);
    return () => { if (--dinleyen === 0) window.removeEventListener("keydown", tus); };
  }, []);
  return useSyncExternalStore(abone, anlik, anlik);
}

export function dilDegistir(): void {
  dilSec(diller[(diller.indexOf(mevcut) + 1) % diller.length]);
}

export function dilSec(dil: Dil): void {
  mevcut = dil;
  try { localStorage.setItem("dil", dil); } catch { /* Bellekte devam et. */ }
  aboneler.forEach(dinleyici => dinleyici());
}

export function bicimle(sablon: string, ...deger: (string | number)[]): string {
  return sablon.replace(/\{(\d+)\}/g, (eslesme, indeks: string) => deger[Number(indeks)]?.toString() ?? eslesme);
}
