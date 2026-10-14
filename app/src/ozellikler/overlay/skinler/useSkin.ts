import { useEffect, useSyncExternalStore } from "react";
import type { Skin } from "../skin";
import { skinler, varsayilanSkin } from ".";

const baslangic = (): Skin => {
  const urlSkin = new URLSearchParams(location.search).get("skin");
  const urlBulunan = skinler.find(skin => skin.ad === urlSkin);
  if (urlBulunan) return urlBulunan;
  try {
    const kayit = localStorage.getItem("skin");
    const bulunan = skinler.find(skin => skin.ad === kayit);
    if (bulunan) return bulunan;
  } catch { /* Varsayilan skin ile devam et. */ }
  return skinler.find(skin => skin.ad === varsayilanSkin) ?? skinler[0];
};

let mevcut = baslangic();
let dinleyen = 0;
const aboneler = new Set<() => void>();
const abone = (dinleyici: () => void) => { aboneler.add(dinleyici); return () => { aboneler.delete(dinleyici); }; };
const anlik = () => mevcut;

export function skinSec(ad: string): void {
  const bulunan = skinler.find(skin => skin.ad === ad);
  if (!bulunan || bulunan.ad === mevcut.ad) return;
  mevcut = bulunan;
  try { localStorage.setItem("skin", ad); } catch { /* Bellekte devam et. */ }
  aboneler.forEach(dinleyici => dinleyici());
}

const degistir = () => {
  const indeks = skinler.findIndex(skin => skin.ad === mevcut.ad);
  skinSec(skinler[(indeks + 1) % skinler.length].ad);
};
const tus = (olay: KeyboardEvent) => {
  if (olay.ctrlKey && olay.shiftKey && olay.key.toLowerCase() === "s") degistir();
};

export function useSkin(): [Skin, () => void, (ad: string) => void] {
  useEffect(() => {
    if (dinleyen++ === 0) window.addEventListener("keydown", tus);
    return () => { if (--dinleyen === 0) window.removeEventListener("keydown", tus); };
  }, []);
  return [useSyncExternalStore(abone, anlik, anlik), degistir, skinSec];
}
