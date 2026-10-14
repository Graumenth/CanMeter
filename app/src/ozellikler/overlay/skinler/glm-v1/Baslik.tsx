import { memo } from "react";
import type { SkinOzellikleri } from "../../skin";
import { Logo } from "./ikonlar";
import { metin } from "./metin";

interface BaslikOzellikleri {
  sure?: string;
  dil: SkinOzellikleri["dil"];
  kilitli: boolean;
  kilitDegistir: () => void;
  ayarlarAc: () => void;
}

export const Baslik = memo(function Baslik({ sure, dil, kilitli, kilitDegistir, ayarlarAc }: BaslikOzellikleri) {
  const m = metin[dil];
  return <header className="titlebar" data-tauri-drag-region>
    <div className="brand"><Logo /><span className="brand-name"><b>Can</b>Meter</span></div>
    {sure && <><span className="tb-sep" /><span className="tb-time">{sure}</span><span className="tb-sep" /><span className="tb-mode">DPS</span></>}
    <span className="tb-flex" />
    <button className="lock" type="button" aria-label={m.ayarlar} onClick={ayarlarAc}>⚙</button>
    <button className="lock" type="button" aria-label={kilitli ? m.kilidiAc : m.kilitle} onClick={kilitDegistir}>{kilitli ? "▣" : "▢"}</button>
  </header>;
});
