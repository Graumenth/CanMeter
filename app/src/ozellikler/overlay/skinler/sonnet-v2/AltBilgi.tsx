import type { SavasAnlik } from "../../../../tipler/savas";
import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";
import { Toplam } from "./Toplam";

interface Ozellikler {
  savas: SavasAnlik;
  dil: SkinOzellikleri["dil"];
  toplamGoster: boolean;
}

export function AltBilgi({ savas, dil, toplamGoster }: Ozellikler) {
  const m = metin[dil];
  return <div className="foot">
    {toplamGoster && <Toplam savas={savas} dil={dil} />}
    <span className="demo"><i />{m.demoVerisi}</span>
    <span>{m.surum}</span>
  </div>;
}
