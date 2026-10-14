import { sure } from "../../../../ortak/sayi";
import type { SavasAnlik } from "../../../../tipler/savas";
import type { SkinOzellikleri } from "../../skin";
import { Hedef } from "./Hedef";
import { metin } from "./metin";

interface Ozellikler {
  savas: SavasAnlik | null;
  dil: SkinOzellikleri["dil"];
  kilitli: boolean;
  kilitDegistir: () => void;
  ayarlarAc: () => void;
}

export function Baslik({ savas, dil, kilitli, kilitDegistir, ayarlarAc }: Ozellikler) {
  const m = metin[dil];
  return <div className="head" data-tauri-drag-region>
    {savas ? <>
      <div className="timer num"><small>{m.sure}</small>{sure(savas.sure_ms)}</div>
      <Hedef hedef={savas.hedef} dil={dil} />
    </> : <strong>{m.canMeter}</strong>}
    <div className="dugmeler" data-tauri-drag-region="false">
      <button className="ikon-dugme" type="button" aria-label={m.ayarlar} onClick={ayarlarAc}>⚙</button>
      <button className="ikon-dugme" type="button" aria-label={kilitli ? m.kilidiAc : m.kilitle}
        onClick={kilitDegistir}>{kilitli ? "▣" : "▢"}</button>
    </div>
  </div>;
}
