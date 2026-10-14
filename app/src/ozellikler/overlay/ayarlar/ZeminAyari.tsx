import { useAyar } from "../../../ortak/ayarlar";
import { useDil } from "../../../ortak/dil";
import { metin } from "./metin";

export function ZeminAyari() {
  const dil = useDil();
  const m = metin[dil];
  const [opaklik, opaklikAyarla] = useAyar("zeminOpaklik", 0.92);
  const [renk, renkAyarla] = useAyar<string | null>("zeminRenk", null);
  return <>
    <label className="ayar-satiri dikey">
      <span>{m.seffaflik} <output>{Math.round(opaklik * 100)}%</output></span>
      <input type="range" min="0.3" max="1" step="0.02" value={opaklik}
        onChange={olay => opaklikAyarla(olay.target.valueAsNumber)} />
    </label>
    <div className="ayar-satiri">
      <label className="renk"><span>{m.zeminRengi}</span>
        <input type="color" value={renk ?? "#0c0f16"} onChange={olay => renkAyarla(olay.target.value)} />
      </label>
      <button type="button" onClick={() => renkAyarla(null)}>{m.varsayilan}</button>
    </div>
  </>;
}
