import { sure } from "../../../../ortak/sayi";
import type { SavasAnlik } from "../../../../tipler/savas";
import type { SkinOzellikleri } from "../../skin";
import { Hedef } from "./Hedef";
import { metin } from "./metin";

interface Ozellikler {
  savas: SavasAnlik | null; dil: SkinOzellikleri["dil"]; kilitli: boolean;
  kilitDegistir: () => void; ayarlarAc?: () => void;
}

export function Baslik({ savas, dil, kilitli, kilitDegistir, ayarlarAc }: Ozellikler) {
  const m = metin[dil];
  return <header className="baslik" data-tauri-drag-region>
    <div className="ust-baslik"><span className="canli" /><span>{m.canMeter}</span>
      {savas && <span className="zaman">{m.savas} {sure(savas.sure_ms)}</span>}
      <div className="dugmeler" data-tauri-drag-region="false">
        <button type="button" aria-label={m.ayarlar} onClick={ayarlarAc}>⚙</button>
        <button type="button" aria-label={kilitli ? m.kilidiAc : m.kilitle} onClick={kilitDegistir}>{kilitli ? "▣" : "▢"}</button>
      </div>
    </div>
    {savas && <Hedef hedef={savas.hedef} dil={dil} />}
  </header>;
}
