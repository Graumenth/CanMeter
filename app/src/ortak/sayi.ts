/// <reference lib="esnext.disposable" />

import type { Dil } from "./dil";

const birimler = ["", "K", "M", "B"] as const;

/** Sayiyi en fazla uc anlamli basamakla kisaltir. */
export function kisalt(n: number): string {
  const isaret = n < 0 ? "-" : "";
  const deger = Math.abs(n);

  if (!Number.isFinite(deger)) return String(n);
  if (deger < 1000) return `${isaret}${Math.round(deger)}`;

  let kademe = Math.min(Math.floor(Math.log10(deger) / 3), birimler.length - 1);
  let basamakli = Number((deger / 1000 ** kademe).toPrecision(3));
  if (basamakli >= 1000 && kademe < birimler.length - 1) { kademe += 1; basamakli = Number((deger / 1000 ** kademe).toPrecision(3)); }
  return `${isaret}${basamakli}${birimler[kademe]}`;
}

/** Oran olarak verilen yuzdeyi tek ondalikla gosterir. */
export function yuzde(p: number, dil: Dil = "tr"): string {
  const deger = (p * 100).toFixed(1);
  return dil === "tr" ? `%${deger}` : `${deger}%`;
}

/** Yuzde degerini en yakin tam sayiya yuvarlayarak gosterir. */
export function yuzdeTam(p: number, dil: Dil = "tr"): string {
  const deger = Math.round(p);
  return dil === "tr" ? `%${deger}` : `${deger}%`;
}

/** Milisaniyeyi dakika:saniye veya saat:dakika:saniye yapar. */
export function sure(ms: number): string {
  const toplamSaniye = Math.max(0, Math.floor(ms / 1000));
  const saniye = toplamSaniye % 60;
  const dakika = Math.floor(toplamSaniye / 60) % 60;
  const saat = Math.floor(toplamSaniye / 3600);
  const ikiHane = (deger: number) => String(deger).padStart(2, "0");

  return saat > 0
    ? `${saat}:${ikiHane(dakika)}:${ikiHane(saniye)}`
    : `${dakika}:${ikiHane(saniye)}`;
}
