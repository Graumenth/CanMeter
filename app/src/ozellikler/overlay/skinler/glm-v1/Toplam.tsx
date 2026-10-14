import { memo } from "react";
import type { SavasAnlik } from "../../../../tipler/savas";
import type { Dil } from "../../../../ortak/dil";
import { kisalt } from "../../../../ortak/sayi";
import { metin } from "./metin";

export const Toplam = memo(function Toplam({ savas, dil }: { savas: SavasAnlik; dil: Dil }) {
  const m = metin[dil];
  const dpsToplam = savas.oyuncular.reduce((toplam, oyuncu) => toplam + oyuncu.dps, 0);
  return <p className="toplam">
    {m.toplamDps} {kisalt(dpsToplam)}
    {" · "}
    {m.hasar} {kisalt(savas.toplam_hasar)}
  </p>;
});
