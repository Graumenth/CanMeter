import { kisalt } from "../../../../ortak/sayi";
import { metin } from "./metin";
import type { SkinOzellikleri } from "../../skin";

interface ToplamOzellikleri {
  dps: number;
  hasar: number;
  dil: SkinOzellikleri["dil"];
}

export function Toplam({ dps, hasar, dil }: ToplamOzellikleri) {
  const m = metin[dil];
  return <span className="toplam">
    {m.toplamDps} {kisalt(dps)} · {m.hasar} {kisalt(hasar)}
  </span>;
}
