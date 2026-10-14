import { memo } from "react";
import type { SkinOzellikleri } from "../../skin";
import { AltBilgi } from "./AltBilgi";
import { Baslik } from "./Baslik";
import { Hedef } from "./Hedef";
import { metin } from "./metin";
import { Satir } from "./Satir";
import { Toplam } from "./Toplam";
import "./skin.css";

export const Sis = memo(function Sis({ savas, kilitli, kilitDegistir, dil, ayarlarAc, toplamGoster }: SkinOzellikleri) {
  const m = metin[dil];
  if (!savas) return <section className="skin-sis"><main className="meter bekleyen"><Baslik dil={dil} savas={null} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} /><p>{m.savasBekleniyor}</p></main></section>;
  const dpsToplam = savas.oyuncular.reduce((toplam, oyuncu) => toplam + oyuncu.dps, 0);
  return <section className="skin-sis"><main className="meter">
    <Baslik dil={dil} savas={savas} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} />
    <Hedef hedef={savas.hedef} dil={dil} />
    {toplamGoster && <Toplam dps={dpsToplam} hasar={savas.toplam_hasar} dil={dil} />}
    <div className="roster" role="table" aria-label={m.oyuncuHasari}>
      <div className="table-row table-heading" role="row"><span className="rank">#</span><span className="class-heading">{m.sinif}</span><span>{m.oyuncu}</span><span className="numeric">{m.dps}</span><span className="numeric">{m.pay}</span></div>
      {savas.oyuncular.map((oyuncu, sira) => <Satir key={oyuncu.id} oyuncu={oyuncu} sira={sira + 1} dil={dil} />)}
    </div><AltBilgi dil={dil} />
  </main></section>;
});
