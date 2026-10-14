import { memo } from "react";
import type { SkinOzellikleri } from "../../skin";
import { sure } from "../../../../ortak/sayi";
import { AltBilgi } from "./AltBilgi";
import { Baslik } from "./Baslik";
import { Hedef } from "./Hedef";
import { Satir } from "./Satir";
import { Toplam } from "./Toplam";
import { metin } from "./metin";
import "./skin.css";

export const Kehribar = memo(function Kehribar({ savas, kilitli, kilitDegistir, dil, ayarlarAc, toplamGoster }: SkinOzellikleri) {
  const m = metin[dil];
  if (!savas) return <section className="skin-kehribar"><div className="panel bekleyen"><Baslik dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} /><p>{m.savasBekleniyor}</p></div></section>;
  return <section className="skin-kehribar"><div className="panel"><Baslik sure={sure(savas.sure_ms)} dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} />
    <Hedef hedef={savas.hedef} dil={dil} />{toplamGoster && <Toplam savas={savas} dil={dil} />}<div className="head"><span className="h">{m.sira}</span><span className="h">{m.sinif}</span><span className="h">{m.oyuncu}</span><span className="h h-dps">{m.dps}<i className="caret" /></span><span className="h h-share">{m.pay}</span></div>
    <ol className="rows">{savas.oyuncular.map((oyuncu, indeks) => <Satir key={oyuncu.id} oyuncu={oyuncu} sira={indeks + 1} dil={dil} />)}</ol><AltBilgi dil={dil} />
  </div></section>;
});
