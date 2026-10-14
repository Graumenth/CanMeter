import { memo, useMemo } from "react";
import { kisalt, yuzde } from "../../ortak/sayi";
import type { CSSProperties } from "react";
import type { OyuncuAnlik, Sinif } from "../../tipler/savas";

const sinifKisaltmalari: Record<Sinif, string> = {
  Gladiator: "GLA", Templar: "TEM", Assassin: "ASS", Ranger: "RAN", Sorcerer: "SOR",
  Spiritmaster: "SPI", Cleric: "CLE", Chanter: "CHA", Bilinmiyor: "BIL",
};

interface OyuncuSatiriOzellikleri { oyuncu: OyuncuAnlik; sira: number; }

export const OyuncuSatiri = memo(function OyuncuSatiri({ oyuncu, sira }: OyuncuSatiriOzellikleri) {
  const pay = Math.max(0, Math.min(1, oyuncu.pay));
  const barStili = useMemo<CSSProperties>(() => ({ width: `${pay * 100}%` }), [pay]);
  const sinif = sinifKisaltmalari[oyuncu.sinif];
  const sinifSinifi = `sinif-${oyuncu.sinif.toLowerCase()}`;
  const satirSinifi = `oyuncu-satiri ${sinifSinifi}${oyuncu.ben ? " ben" : ""}${oyuncu.olu ? " olu" : ""}`;

  return (
    <article className={satirSinifi}>
      <div className="oyuncu-veri">
        <span className="sira">{sira}</span><b className="sinif-kisa">{sinif}</b>
        <span className="oyuncu-adi">{oyuncu.ad} <small>{oyuncu.sinif}</small>{oyuncu.olu && " ✝"}</span>
        <strong className="dps">{kisalt(oyuncu.dps)}</strong><span className="pay">{yuzde(pay)}</span>
      </div>
      <i className="bar" style={barStili} />
    </article>
  );
});
