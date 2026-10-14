import { useCallback, useSyncExternalStore } from "react";

const aboneler = new Set<() => void>();
const degerler = new Map<string, unknown>();
const abone = (dinleyici: () => void) => {
  aboneler.add(dinleyici);
  return () => { aboneler.delete(dinleyici); };
};

function oku<T>(anahtar: string, varsayilan: T): T {
  if (degerler.has(anahtar)) return degerler.get(anahtar) as T;
  let deger = varsayilan;
  try {
    const kayit = localStorage.getItem(`ayar.${anahtar}`);
    if (kayit !== null) deger = JSON.parse(kayit) as T;
  } catch { /* Gecersiz veya erisilemeyen kayitta varsayilani kullan. */ }
  degerler.set(anahtar, deger);
  return deger;
}

export function useAyar<T>(anahtar: string, varsayilan: T): [T, (deger: T) => void] {
  const anlik = useCallback(() => oku(anahtar, varsayilan), [anahtar, varsayilan]);
  const ayarla = useCallback((deger: T) => {
    degerler.set(anahtar, deger);
    try { localStorage.setItem(`ayar.${anahtar}`, JSON.stringify(deger)); } catch { /* Bellekte devam et. */ }
    aboneler.forEach(dinleyici => dinleyici());
  }, [anahtar]);
  return [useSyncExternalStore(abone, anlik, anlik), ayarla];
}
