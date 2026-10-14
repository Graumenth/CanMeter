import { memo } from "react";
import type { CSSProperties } from "react";
import type { Dil } from "../../../../ortak/dil";
import { kisalt, yuzde } from "../../../../ortak/sayi";
import { sinifRenk } from "../../../../ortak/sinifRenk";
import type { OyuncuAnlik } from "../../../../tipler/savas";
import { SinifIkonu } from "./ikonlar";
import { metin } from "./metin";

const satirStili = (renk: string, hasar: number, enYuksek: number) => ({
  "--c": renk, "--w": `${Math.max(0, Math.min(100, hasar / enYuksek * 100))}%`,
}) as CSSProperties;

interface SatirOzellikleri { oyuncu: OyuncuAnlik; sira: number; enYuksek: number; dil: Dil; }

export const Satir = memo(function Satir({ oyuncu, sira, enYuksek, dil }: SatirOzellikleri) {
  const m = metin[dil];
  const sinif = sinifRenk[oyuncu.sinif] ?? sinifRenk.Bilinmiyor;
  const sinifAdi = oyuncu.sinif === "Bilinmiyor" ? "?" : oyuncu.sinif;
  return <div className={`row${oyuncu.ben ? " row--me" : ""}${oyuncu.olu ? " row--dead" : ""}`} style={satirStili(sinif, oyuncu.hasar, enYuksek)}>
    <span className="row__rank">{sira}</span><span className="row__who"><SinifIkonu sinif={oyuncu.sinif} />
      {oyuncu.olu && <span className="skull">☠</span>}<span className="row__name">{oyuncu.ad}</span>
      {oyuncu.ben && <span className="tag tag--me">{m.ben}</span>}{oyuncu.iyilestirme > 0 && <span className="tag tag--heal">+{kisalt(oyuncu.hps)} {m.hps}</span>}
      <span className="row__class">{sinifAdi}</span></span><span className="row__dps">{kisalt(oyuncu.dps)}</span>
    <span className="row__pct">{yuzde(oyuncu.pay, dil)}</span><span className="row__bar" />
  </div>;
});
