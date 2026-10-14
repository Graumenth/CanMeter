import { memo } from "react";
import { kisalt } from "../../../../ortak/sayi";
import type { SavasAnlik } from "../../../../tipler/savas";
import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";

interface Ozellikler { savas: SavasAnlik; dil: SkinOzellikleri["dil"]; }

export const Toplam = memo(function Toplam({ savas, dil }: Ozellikler) {
  const m = metin[dil];
  const dpsToplam = savas.oyuncular.reduce((toplam, oyuncu) => toplam + oyuncu.dps, 0);
  return <section className="toplam-ozeti" aria-label={`${m.toplamDps}: ${kisalt(dpsToplam)}, ${m.hasar}: ${kisalt(savas.toplam_hasar)}`}>
    <div className="toplam-deger"><small>{m.toplamDps}</small><strong>{kisalt(dpsToplam)}</strong></div>
    <div className="toplam-deger"><small>{m.hasar}</small><strong>{kisalt(savas.toplam_hasar)}</strong></div>
  </section>;
});
