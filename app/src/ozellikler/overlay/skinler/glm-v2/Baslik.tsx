import type { Dil } from "../../../../ortak/dil";
import { sure } from "../../../../ortak/sayi";
import { Logo } from "./ikonlar";
import { metin } from "./metin";

interface Ozellikler { dil: Dil; sureMs?: number; kilitli: boolean; kilitDegistir: () => void; ayarlarAc?: () => void; }

export function Baslik({ dil, sureMs, kilitli, kilitDegistir, ayarlarAc }: Ozellikler) {
  const m = metin[dil];
  return <header className="head" data-tauri-drag-region>
    <span className="marka"><Logo /><span className="wordmark"><b>CAN</b>METER</span></span><span className="spacer" />
    {sureMs != null && <><span className="badge"><i />{m.demoVerisi}</span><span className="timer"><small>{m.sure}</small><b>{sure(sureMs)}</b></span></>}
    <button className="arac" type="button" aria-label={m.ayarlar} onClick={ayarlarAc}>⚙</button>
    <button className="arac" type="button" aria-label={kilitli ? m.kilidiAc : m.kilitle} onClick={kilitDegistir}>{kilitli ? "▣" : "▢"}</button>
  </header>;
}
