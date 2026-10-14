import { metin } from "./metin";
import { MarkaIkonu } from "./ikonlar";
import type { SkinOzellikleri } from "../../skin";
import { sure } from "../../../../ortak/sayi";

type BaslikOzellikleri = Pick<SkinOzellikleri, "savas" | "dil" | "kilitli" | "kilitDegistir" | "ayarlarAc">;

export function Baslik({ savas, dil, kilitli, kilitDegistir, ayarlarAc }: BaslikOzellikleri) {
  const m = metin[dil];
  return <header className="meter-header" data-tauri-drag-region>
    <h1 className="brand" data-tauri-drag-region><MarkaIkonu />{m.canMeter}</h1>
    <span className="header-spacer" data-tauri-drag-region />
    {savas && <span className="combat-time" aria-label={`${m.savasSuresi} ${sure(savas.sure_ms)}`}>◷ {sure(savas.sure_ms)}</span>}
    {savas && <span className="mode">{m.dps}</span>}
    <button className="kontrol" type="button" aria-label={m.ayarlar} onClick={ayarlarAc}>⚙</button>
    <button className="kontrol kilit" type="button" aria-label={kilitli ? m.kilidiAc : m.kilitle} onClick={kilitDegistir}>{kilitli ? "▣" : "▢"}</button>
  </header>;
}
