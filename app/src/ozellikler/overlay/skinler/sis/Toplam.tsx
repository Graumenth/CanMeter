import { memo } from "react";
import { kisalt } from "../../../../ortak/sayi";
import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";

interface ToplamOzellikleri {
  dps: number;
  hasar: number;
  dil: SkinOzellikleri["dil"];
}

export const Toplam = memo(function Toplam({ dps, hasar, dil }: ToplamOzellikleri) {
  const m = metin[dil];
  return <section className="toplam" aria-label={m.toplamDps}>
    <div className="toplam-deger"><span className="toplam-etiket">{m.toplamDps}</span><strong>{kisalt(dps)}</strong></div>
    <div className="toplam-deger"><span className="toplam-etiket">{m.hasar}</span><strong>{kisalt(hasar)}</strong></div>
  </section>;
});
