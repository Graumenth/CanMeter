import type { CSSProperties } from "react";
import type { OyuncuAnlik, Sinif } from "../../../../tipler/savas";
import { kisalt, yuzde } from "../../../../ortak/sayi";
import { sinifRenk } from "../../../../ortak/sinifRenk";
import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";

function SinifIkonu({ sinif }: { sinif: Sinif }) {
  const ortak = { className: "ic", viewBox: "0 0 22 22", "aria-hidden": true };
  if (sinif === "Gladiator") return <svg {...ortak}><path d="M11 2l8 5v8l-8 5-8-5V7z" fill="none" stroke="currentColor" strokeWidth="1.8"/><path d="M11 7v8M7.5 11h7" stroke="currentColor" strokeWidth="1.6"/></svg>;
  if (sinif === "Sorcerer") return <svg {...ortak}><circle cx="11" cy="11" r="7.5" fill="none" stroke="currentColor" strokeWidth="1.8"/><path d="M11 5l2 4.5 4.5.5-3.4 3 1 4.5L11 15l-4.1 2.5 1-4.5-3.4-3 4.5-.5z" fill="currentColor"/></svg>;
  if (sinif === "Assassin") return <svg {...ortak}><path d="M4 18L18 4M18 4h-6M18 4v6" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/><path d="M4 12l6 6" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round"/></svg>;
  if (sinif === "Chanter") return <svg {...ortak}><path d="M4 11c2-5 4-5 7 0s5 5 7 0" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/><circle cx="11" cy="17" r="1.6" fill="currentColor"/></svg>;
  if (sinif === "Templar") return <svg {...ortak}><path d="M11 2l7 3v6c0 4-3 7-7 9-4-2-7-5-7-9V5z" fill="none" stroke="currentColor" strokeWidth="1.8"/></svg>;
  if (sinif === "Cleric") return <svg {...ortak}><path d="M9 3h4v6h6v4h-6v6H9v-6H3V9h6z" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinejoin="round"/></svg>;
  return <svg {...ortak}><circle cx="11" cy="11" r="7" fill="none" stroke="currentColor" strokeWidth="1.8"/><path d="M7 11h8M11 7v8" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round"/></svg>;
}

export function Satir({ oyuncu, enYuksekHasar, dil }: { oyuncu: OyuncuAnlik; enYuksekHasar: number; dil: SkinOzellikleri["dil"] }) {
  const m = metin[dil];
  const stil = { "--c": sinifRenk[oyuncu.sinif] } as CSSProperties;
  const genislik = `${Math.max(0, Math.min(100, oyuncu.hasar / enYuksekHasar * 100))}%`;
  return <div className={`p${oyuncu.ben ? " me" : ""}${oyuncu.olu ? " dead" : ""}`} style={stil}>
    <SinifIkonu sinif={oyuncu.sinif} />
    <div className="who"><div className="nm"><span>{oyuncu.ad}</span>{oyuncu.ben && <span className="tag me">{m.sen}</span>}<span className="cl">{oyuncu.sinif}</span>{oyuncu.olu && <span className="tag dead">{m.oldu}</span>}{oyuncu.iyilestirme > 0 && <span className="tag heal num">+{kisalt(oyuncu.hps)} {m.hps}</span>}</div><div className="trail"><i style={{ width: genislik }} /></div></div>
    <div className="val"><b className="num">{kisalt(oyuncu.dps)}</b><span className="num">{yuzde(oyuncu.pay, dil)}</span></div>
  </div>;
}
