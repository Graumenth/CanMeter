import { useDil } from "../../../ortak/dil";
import { skinler } from "../skinler";
import { useSkin } from "../skinler/useSkin";
import { metin } from "./metin";

export function SkinSecimi() {
  const dil = useDil();
  const [skin, , sec] = useSkin();
  return <label className="ayar-satiri">
    <span>{metin[dil].gorunum}</span>
    <select value={skin.ad} onChange={olay => sec(olay.target.value)}>
      {skinler.map(secenek => <option key={secenek.ad} value={secenek.ad}>{secenek.etiket}</option>)}
    </select>
  </label>;
}
