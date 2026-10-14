import { memo } from "react";
import type { SkinOzellikleri } from "../../skin";
import { sure } from "../../../../ortak/sayi";
import { AltBilgi } from "./AltBilgi";
import { Baslik } from "./Baslik";
import { Hedef } from "./Hedef";
import { metin } from "./metin";
import { Satir } from "./Satir";
import { Toplam } from "./Toplam";
import "./skin.css";

type KorOzellikleri = SkinOzellikleri;

export const Kor = memo(function Kor({ savas, kilitli, kilitDegistir, dil, ayarlarAc, toplamGoster }: KorOzellikleri) {
  const m = metin[dil];
  if (!savas) return <section className="skin-kor"><main className="meter bekleyen">
    <Baslik dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} /><p>{m.savasBekleniyor}</p>
  </main></section>;
  const enYuksek = Math.max(1, ...savas.oyuncular.map(({ hasar }) => hasar));
  const dpsToplam = savas.oyuncular.reduce((toplam, { dps }) => toplam + dps, 0);
  return <section className="skin-kor"><main className="meter">
    <Baslik dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir} ayarlarAc={ayarlarAc} sure={sure(savas.sure_ms)} />
    {savas.hedef && <Hedef hedef={savas.hedef} dil={dil} />}
    {toplamGoster && <Toplam dil={dil} dps={dpsToplam} hasar={savas.toplam_hasar} />}
    <div className="cols"><span>#</span><span>{m.oyuncu}</span><span className="num">{m.dps}</span><span className="num">{m.pay}</span></div>
    <div className="rows">{savas.oyuncular.map((oyuncu, indeks) => <Satir key={oyuncu.id} oyuncu={oyuncu} sira={indeks + 1} enYuksek={enYuksek} dil={dil} />)}</div>
    <AltBilgi dil={dil} />
  </main></section>;
});
