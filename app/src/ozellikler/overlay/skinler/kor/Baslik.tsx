import type { Dil } from "../../../../ortak/dil";
import { metin } from "./metin";

interface BaslikOzellikleri {
  dil: Dil; kilitli: boolean; kilitDegistir: () => void; ayarlarAc?: () => void; sure?: string;
}

export function Baslik({ dil, kilitli, kilitDegistir, ayarlarAc, sure }: BaslikOzellikleri) {
  const m = metin[dil];
  return <header className="meter__head" data-tauri-drag-region>
    <div className="brand"><span className="brand__mark" />{m.canMeter}</div>
    {sure && <span className="timer">{sure}</span>}
    <span className="mode">{m.dps}</span><span className="spacer" />
    <button className="lock" type="button" aria-label={m.ayarlar} onClick={ayarlarAc}>⚙</button>
    <button className="lock" type="button" aria-label={kilitli ? m.kilidiAc : m.kilitle} onClick={kilitDegistir}>
      {kilitli ? "▣" : "▢"}
    </button>
  </header>;
}
