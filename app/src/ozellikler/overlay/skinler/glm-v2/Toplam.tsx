import type { Dil } from "../../../../ortak/dil";
import { kisalt } from "../../../../ortak/sayi";
import { metin } from "./metin";

interface Ozellikler {
  dil: Dil;
  dps: number;
  hasar: number;
}

export function Toplam({ dil, dps, hasar }: Ozellikler) {
  const m = metin[dil];
  return <p className="toplam">
    {m.toplamDps} {kisalt(dps)} · {m.hasar} {kisalt(hasar)}
  </p>;
}
