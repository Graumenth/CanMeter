import { memo } from "react";
import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";

interface Ozellikler { dil: SkinOzellikleri["dil"]; }

export const AltBilgi = memo(function AltBilgi({ dil }: Ozellikler) {
  const m = metin[dil];
  return <footer className="alt-bilgi"><span className="demo">{m.demoVerisi}</span>
    <span className="surum">{m.surum}</span>
  </footer>;
});
