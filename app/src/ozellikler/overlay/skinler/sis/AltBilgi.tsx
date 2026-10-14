import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";

type AltBilgiOzellikleri = Pick<SkinOzellikleri, "dil">;

export function AltBilgi({ dil }: AltBilgiOzellikleri) {
  const m = metin[dil];
  return <footer className="meter-footer">
    <span className="demo-status"><i className="demo-dot" />{m.demoVerisi}</span>
    <span className="version">{m.surum}</span>
  </footer>;
}
