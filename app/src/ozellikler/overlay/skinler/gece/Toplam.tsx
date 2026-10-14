import { memo } from "react";
import { kisalt } from "../../../../ortak/sayi";
import type { SavasAnlik } from "../../../../tipler/savas";
import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";

interface Ozellikler {
  savas: SavasAnlik;
  dil: SkinOzellikleri["dil"];
}

export const Toplam = memo(function Toplam({ savas, dil }: Ozellikler) {
  const m = metin[dil];
  const dpsToplam = savas.oyuncular.reduce((toplam, oyuncu) => toplam + oyuncu.dps, 0);
  return <div className="toplam">
    <div><span>{m.toplamDps}</span><b className="num">{kisalt(dpsToplam)}</b></div>
    <div><span>{m.hasar}</span><b className="num">{kisalt(savas.toplam_hasar)}</b></div>
  </div>;
});
