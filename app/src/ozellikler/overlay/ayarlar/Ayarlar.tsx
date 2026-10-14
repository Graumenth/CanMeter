import { memo, useEffect, useRef } from "react";
import { useDil } from "../../../ortak/dil";
import { DilSecimi } from "./DilSecimi";
import { SkinSecimi } from "./SkinSecimi";
import { ToplamAyari } from "./ToplamAyari";
import { ZeminAyari } from "./ZeminAyari";
import { metin } from "./metin";
import "./ayarlar.css";

export const Ayarlar = memo(function Ayarlar({ acik, kapat }: { acik: boolean; kapat: () => void }) {
  const dil = useDil();
  const kart = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!acik) return;
    const tus = (olay: KeyboardEvent) => { if (olay.key === "Escape") kapat(); };
    const disari = (olay: PointerEvent) => {
      if (!kart.current?.contains(olay.target as Node)) kapat();
    };
    document.addEventListener("keydown", tus);
    document.addEventListener("pointerdown", disari);
    return () => {
      document.removeEventListener("keydown", tus);
      document.removeEventListener("pointerdown", disari);
    };
  }, [acik, kapat]);
  if (!acik) return null;
  const m = metin[dil];
  return <div className="ayarlar" ref={kart} role="dialog" aria-modal="true" aria-label={m.ayarlar}>
    <header><strong>{m.ayarlar}</strong><button type="button" aria-label={m.kapat} onClick={kapat}>×</button></header>
    <DilSecimi />
    <SkinSecimi />
    <ZeminAyari />
    <ToplamAyari />
    <div className="kisayollar"><b>{m.kisayollar}</b><span>{m.gorunumKisayol}</span><span>{m.dilKisayol}</span></div>
  </div>;
});
