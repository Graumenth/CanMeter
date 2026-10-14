import { metin } from "./metin";
import type { SkinOzellikleri } from "../../skin";
import { Toplam } from "./Toplam";

interface AltBilgiOzellikleri {
  dil: SkinOzellikleri["dil"];
  toplamGoster: boolean;
  dps: number;
  hasar: number;
}

export function AltBilgi({ dil, toplamGoster, dps, hasar }: AltBilgiOzellikleri) {
  const m = metin[dil];
  return <footer className="meter-footer">
    <span className="demo-status"><i className="demo-dot" />{m.demoVerisi}</span>
    {toplamGoster && <Toplam dil={dil} dps={dps} hasar={hasar} />}
    <span className="version">{m.surum}</span>
  </footer>;
}
