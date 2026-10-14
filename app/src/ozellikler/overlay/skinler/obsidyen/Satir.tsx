import { memo } from "react";
import type { CSSProperties } from "react";
import { kisalt, yuzde } from "../../../../ortak/sayi";
import { sinifRenk } from "../../../../ortak/sinifRenk";
import type { OyuncuAnlik } from "../../../../tipler/savas";
import type { SkinOzellikleri } from "../../skin";
import { SinifIkonu, SinifKisaltmasi } from "./ikonlar";
import { metin } from "./metin";

interface Ozellikler { oyuncu: OyuncuAnlik; sira: number; enYuksekHasar: number; dil: SkinOzellikleri["dil"]; }

export const Satir = memo(function Satir({ oyuncu, sira, enYuksekHasar, dil }: Ozellikler) {
  const m = metin[dil];
  const genislik = Math.max(0, Math.min(100, oyuncu.hasar / enYuksekHasar * 100));
  const stil = { "--rol": sinifRenk[oyuncu.sinif] ?? sinifRenk.Bilinmiyor, "--genislik": `${genislik}%` } as CSSProperties;
  return <article className={`oyuncu${oyuncu.ben ? " ben" : ""}${oyuncu.olu ? " olu" : ""}`} style={stil}>
    <div className="sira">{String(sira + 1).padStart(2, "0")}</div><div className="kimlik"><div className="ad-satiri">
      <span className="ad">{oyuncu.ad}</span>{oyuncu.ben && <span className="etiket">{m.ben}</span>}
    </div><div className="sinif-notu"><SinifIkonu sinif={oyuncu.sinif} /><span>{oyuncu.sinif}</span><SinifKisaltmasi sinif={oyuncu.sinif} />
      {oyuncu.iyilestirme > 0 && <span className="iyilestirme">+ {kisalt(oyuncu.hps)} {m.hps}</span>}{oyuncu.olu && <span className="oldu">{m.oldu}</span>}
    </div></div><div className="dps">{kisalt(oyuncu.dps)} <small>{m.dps}</small></div><div className="pay">{yuzde(oyuncu.pay, dil)}</div>
  </article>;
});
