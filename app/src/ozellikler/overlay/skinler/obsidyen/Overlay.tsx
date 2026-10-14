import { memo } from "react";
import { bicimle } from "../../../../ortak/dil";
import type { SkinOzellikleri } from "../../skin";
import { AltBilgi } from "./AltBilgi";
import { Baslik } from "./Baslik";
import { metin } from "./metin";
import { Satir } from "./Satir";
import { Toplam } from "./Toplam";
import "./skin.css";

type ObsidyenOzellikleri = SkinOzellikleri;

export const Obsidyen = memo(function Obsidyen({ savas, kilitli, kilitDegistir, dil, ayarlarAc, toplamGoster }: ObsidyenOzellikleri) {
  const m = metin[dil];
  if (!savas) return <section className="skin-obsidyen"><div className="panel bekleyen">
    <Baslik savas={null} dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} />
    <p>{m.savasBekleniyor}</p>
  </div></section>;
  const enYuksekHasar = Math.max(1, ...savas.oyuncular.map(oyuncu => oyuncu.hasar));
  return <section className="skin-obsidyen"><div className="panel"><Baslik savas={savas} dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} />
    {toplamGoster && <Toplam savas={savas} dil={dil} />}
    <div className="bolum-baslik" aria-hidden="true"><span>#</span><span>{bicimle(m.oyuncuSayisi, savas.oyuncular.length)}</span><span>{m.dps}</span><span>{m.pay}</span></div>
    <section className="liste" aria-label={m.oyuncuSayisi}>{savas.oyuncular.map((oyuncu, sira) => <Satir key={oyuncu.id} oyuncu={oyuncu} sira={sira} enYuksekHasar={enYuksekHasar} dil={dil} />)}</section>
    <AltBilgi dil={dil} />
  </div></section>;
});
