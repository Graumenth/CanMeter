import { memo } from "react";
import { bicimle } from "../../../../ortak/dil";
import { sinifRenk } from "../../../../ortak/sinifRenk";
import type { SkinOzellikleri } from "../../skin";
import { AltBilgi } from "./AltBilgi";
import { Baslik } from "./Baslik";
import { metin } from "./metin";
import { Satir } from "./Satir";
import "./skin.css";

export const SonnetV2 = memo(function SonnetV2(ozellikler: SkinOzellikleri) {
  const { savas, dil, kilitli, kilitDegistir, ayarlarAc, toplamGoster } = ozellikler;
  const m = metin[dil];
  if (!savas) return <section className="skin-sonnet-v2"><div className="panel bekleyen">
    <Baslik savas={null} dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} />
    <p>{m.savasBekleniyor}</p>
  </div></section>;
  const enYuksekHasar = Math.max(1, ...savas.oyuncular.map(oyuncu => oyuncu.hasar));
  return <section className="skin-sonnet-v2"><div className="panel">
    <Baslik savas={savas} dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} />
    <div className="share"><div className="strip">{savas.oyuncular.map(oyuncu => <i key={oyuncu.id} style={{ flex: Math.max(0, oyuncu.pay), background: sinifRenk[oyuncu.sinif], opacity: oyuncu.olu ? .5 : 1 }} />)}</div><div className="strip-lbl"><span>{m.partiPayi}</span><span>{bicimle(m.oyuncuSayisi, savas.oyuncular.length)}</span></div></div>
    <div className="list">{savas.oyuncular.map(oyuncu => <Satir key={oyuncu.id} oyuncu={oyuncu} enYuksekHasar={enYuksekHasar} dil={dil} />)}</div>
    <AltBilgi savas={savas} dil={dil} toplamGoster={toplamGoster} />
  </div></section>;
});
