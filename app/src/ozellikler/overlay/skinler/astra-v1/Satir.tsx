import { memo } from "react";
import type { CSSProperties } from "react";
import { kisalt, yuzde } from "../../../../ortak/sayi";
import { sinifRenk } from "../../../../ortak/sinifRenk";
import type { OyuncuAnlik } from "../../../../tipler/savas";
import { SinifIkonu } from "./ikonlar";
import { metin } from "./metin";
import type { SkinOzellikleri } from "../../skin";

interface SatirOzellikleri { oyuncu: OyuncuAnlik; sira: number; dil: SkinOzellikleri["dil"]; }

export const Satir = memo(function Satir({ oyuncu, sira, dil }: SatirOzellikleri) {
  const m = metin[dil];
  const stil = { "--sinif-renk": sinifRenk[oyuncu.sinif], "--pay": `${Math.max(0, Math.min(100, oyuncu.pay * 100))}%` } as CSSProperties;
  return <div className={`table-row player-row${oyuncu.ben ? " is-self" : ""}${oyuncu.olu ? " is-dead" : ""}`} style={stil} role="row">
    <span className="rank" role="cell">{sira}</span><span className="class-icon" role="cell" aria-label={oyuncu.sinif} title={oyuncu.sinif}><SinifIkonu sinif={oyuncu.sinif} /></span>
    <div className="player-details" role="cell"><div className="player-line"><span className="player-name">{oyuncu.ad}</span>{oyuncu.ben && <span className="self-badge">{m.ben}</span>}{oyuncu.olu && <span className="death-status">☠ {m.olu}</span>}</div>
      {oyuncu.iyilestirme > 0 && <span className="healing">{kisalt(oyuncu.hps)} {m.hps}</span>}</div>
    <span className="dps" role="cell">{kisalt(oyuncu.dps)}</span><span className="share" role="cell">{yuzde(oyuncu.pay, dil)}</span>
  </div>;
});
