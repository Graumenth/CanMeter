import { memo } from "react";
import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";

interface Ozellikler {
  dil: SkinOzellikleri["dil"];
}

export const AltBilgi = memo(function AltBilgi({ dil }: Ozellikler) {
  const m = metin[dil];
  return <div className="foot">
    <span className="demo"><i />{m.demoVerisi}</span>
    <span>{m.surum}</span>
  </div>;
});
