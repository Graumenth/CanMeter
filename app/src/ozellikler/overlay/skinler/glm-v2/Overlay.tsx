import { memo } from "react";
import type { SkinOzellikleri } from "../../skin";
import { AltBilgi } from "./AltBilgi";
import { Baslik } from "./Baslik";
import { Hedef } from "./Hedef";
import { Satir } from "./Satir";
import { Toplam } from "./Toplam";
import { metin } from "./metin";
import "./skin.css";

type GlmOzellikleri = SkinOzellikleri;

export const GlmV2 = memo(function GlmV2({ savas, kilitli, kilitDegistir, dil, ayarlarAc, toplamGoster }: GlmOzellikleri) {
  const m = metin[dil];
  if (!savas) return <section className="skin-glm-v2"><main className="panel bekleyen"><Baslik dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} /><p>{m.savasBekleniyor}</p></main></section>;
  const enYuksekHasar = Math.max(1, ...savas.oyuncular.map(oyuncu => oyuncu.hasar));
  const dpsToplam = savas.oyuncular.reduce((toplam, oyuncu) => toplam + oyuncu.dps, 0);
  return <section className="skin-glm-v2"><main className="panel"><Baslik dil={dil} sureMs={savas.sure_ms} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} />
    <Hedef hedef={savas.hedef} oyuncuSayisi={savas.oyuncular.length} dil={dil} /><div className="cols"><span /><span /><span>{m.oyuncu}</span><b>{m.dps}</b><b>{m.pay}</b></div>
    <ol className="roster">{savas.oyuncular.map((oyuncu, indeks) => <Satir key={oyuncu.id} oyuncu={oyuncu} sira={indeks + 1} enYuksekHasar={enYuksekHasar} dil={dil} />)}</ol>
    {toplamGoster && <Toplam dil={dil} dps={dpsToplam} hasar={savas.toplam_hasar} />}<AltBilgi dil={dil} />
  </main></section>;
});
