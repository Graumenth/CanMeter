import { memo } from "react";
import type { Dil } from "../../../../ortak/dil";
import { kisalt } from "../../../../ortak/sayi";
import { metin } from "./metin";

interface ToplamOzellikleri {
  dil: Dil;
  dps: number;
  hasar: number;
}

export const Toplam = memo(function Toplam({ dil, dps, hasar }: ToplamOzellikleri) {
  const m = metin[dil];
  return <section className="toplam" aria-label={m.toplamDps}>
    <div className="toplam__istatistik">
      <span className="toplam__etiket">{m.toplamDps}</span>
      <strong className="toplam__deger">{kisalt(dps)}</strong>
    </div>
    <div className="toplam__istatistik toplam__istatistik--sag">
      <span className="toplam__etiket">{m.hasar}</span>
      <strong className="toplam__deger">{kisalt(hasar)}</strong>
    </div>
  </section>;
});
