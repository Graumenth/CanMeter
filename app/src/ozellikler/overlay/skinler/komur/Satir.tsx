import { memo } from "react";
import type { CSSProperties } from "react";
import type { Dil } from "../../../../ortak/dil";
import { kisalt, yuzde } from "../../../../ortak/sayi";
import { sinifRenk } from "../../../../ortak/sinifRenk";
import type { OyuncuAnlik } from "../../../../tipler/savas";
import { Kafatas, SinifIkonu } from "./ikonlar";
import { metin } from "./metin";

const stil = (renk: string, genislik: number) => ({ "--c": renk, "--w": `${genislik}%` }) as CSSProperties;

export const Satir = memo(function Satir({ oyuncu, sira, enYuksekHasar, dil }: { oyuncu: OyuncuAnlik; sira: number; enYuksekHasar: number; dil: Dil }) {
  const m = metin[dil];
  const genislik = Math.max(0, Math.min(100, oyuncu.hasar / enYuksekHasar * 100));
  return <li className={`row ${oyuncu.ben ? "me " : ""}${oyuncu.olu ? "dead" : ""}`} style={stil(sinifRenk[oyuncu.sinif], genislik)}>
    <i className="fill" /><span className="rank">{sira}</span><span className="dot"><SinifIkonu sinif={oyuncu.sinif} /></span>
    <span className="who"><b>{oyuncu.ad}</b><em>{oyuncu.sinif}</em>{oyuncu.ben && <small className="chip me-c">{m.sen}</small>}
      {oyuncu.iyilestirme > 0 && <small className="chip heal">+{kisalt(oyuncu.hps)} {m.hps}</small>}{oyuncu.olu && <small className="chip dead-c"><Kafatas />{m.oldu}</small>}</span>
    <strong className="dps">{kisalt(oyuncu.dps)}</strong><span className="share">{yuzde(oyuncu.pay, dil)}</span>
  </li>;
});
