import { memo } from "react";
import type { SavasAnlik } from "../../../../tipler/savas";
import type { Dil } from "../../../../ortak/dil";
import { kisalt } from "../../../../ortak/sayi";
import { metin } from "./metin";

export const Toplam = memo(function Toplam({ savas, dil }: { savas: SavasAnlik; dil: Dil }) {
  const m = metin[dil];
  const dpsToplam = savas.oyuncular.reduce((toplam, oyuncu) => toplam + oyuncu.dps, 0);
  return <section className="toplam" aria-label={m.toplamDps}>
    <div className="toplam-deger">
      <span className="toplam-etiket">{m.toplamDps}</span>
      <strong>{kisalt(dpsToplam)}</strong>
    </div>
    <div className="toplam-deger toplam-hasar">
      <span className="toplam-etiket">{m.hasar}</span>
      <strong>{kisalt(savas.toplam_hasar)}</strong>
    </div>
  </section>;
});
