import { memo } from "react";
import type { OyuncuAnlik } from "../../../../tipler/savas";
import type { Dil } from "../../../../ortak/dil";
import { kisalt, yuzde } from "../../../../ortak/sayi";
import { SinifIkonu } from "./ikonlar";
import { metin } from "./metin";

export const Satir = memo(function Satir({ oyuncu, sira, dil }: { oyuncu: OyuncuAnlik; sira: number; dil: Dil }) {
  const m = metin[dil];
  const sinifSinifi = oyuncu.sinif.toLowerCase();
  const payGenisligi = `${Math.max(0, Math.min(1, oyuncu.pay)) * 100}%`;
  return <li className={`row sinif-${sinifSinifi}${oyuncu.ben ? " me" : ""}${oyuncu.olu ? " dead" : ""}`}>
    <span className="rank">{sira}</span><span className="cls" title={oyuncu.sinif}><SinifIkonu sinif={oyuncu.sinif} /><span className="cls-name">{oyuncu.sinif}</span></span>
    <span className="who"><span className="nm">{oyuncu.ad}</span>{oyuncu.ben && <span className="me-tag">{m.ben}</span>}
      {oyuncu.iyilestirme > 0 && <span className="heal"><span>+</span><span className="hw">{m.iyilestirme}</span><b>{kisalt(oyuncu.hps)} {m.hps}</b></span>}{oyuncu.olu && <span className="dead-tag">☠ {m.oldu}</span>}</span>
    <span className="dps">{kisalt(oyuncu.dps)}</span><span className="share">{yuzde(oyuncu.pay, dil)}</span><span className="meter"><span className="fill" style={{ width: payGenisligi }} /></span>
  </li>;
});
