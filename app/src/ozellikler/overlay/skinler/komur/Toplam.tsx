import { memo } from "react";
import type { Dil } from "../../../../ortak/dil";
import { kisalt } from "../../../../ortak/sayi";
import { metin } from "./metin";

interface Ozellikler {
  dil: Dil;
  dps: number;
  hasar: number;
}

export const Toplam = memo(function Toplam({ dil, dps, hasar }: Ozellikler) {
  const m = metin[dil];
  return <section className="toplam">
    <span className="toplam-deger"><small>{m.toplamDps}</small><b>{kisalt(dps)}</b></span>
    <span className="toplam-deger"><small>{m.hasar}</small><b>{kisalt(hasar)}</b></span>
  </section>;
});
