import { memo } from "react";
import type { SkinOzellikleri } from "../../skin";
import { AltBilgi } from "./AltBilgi";
import { Baslik } from "./Baslik";
import { metin } from "./metin";
import { Satir } from "./Satir";
import { Toplam } from "./Toplam";
import "./skin.css";

export const Gece = memo(function Gece(ozellikler: SkinOzellikleri) {
  const { savas, dil, kilitli, kilitDegistir, ayarlarAc, toplamGoster } = ozellikler;
  const m = metin[dil];
  if (!savas) return <section className="skin-gece"><div className="panel bekleyen">
    <Baslik savas={null} dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} />
    <p>{m.savasBekleniyor}</p>
  </div></section>;
  const enYuksekHasar = Math.max(1, ...savas.oyuncular.map(oyuncu => oyuncu.hasar));
  return <section className="skin-gece"><div className="panel">
    <Baslik savas={savas} dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} />
    {toplamGoster && <Toplam savas={savas} dil={dil} />}
    <div className="list">{savas.oyuncular.map(oyuncu => <Satir key={oyuncu.id} oyuncu={oyuncu} enYuksekHasar={enYuksekHasar} dil={dil} />)}</div>
    <AltBilgi dil={dil} />
  </div></section>;
});
