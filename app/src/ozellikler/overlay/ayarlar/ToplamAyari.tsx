import { useAyar } from "../../../ortak/ayarlar";
import { useDil } from "../../../ortak/dil";
import { metin } from "./metin";

export function ToplamAyari() {
  const dil = useDil();
  const [goster, gosterAyarla] = useAyar("toplamGoster", true);
  return <label className="ayar-satiri onay">
    <span>{metin[dil].toplamGostergesi}</span>
    <input type="checkbox" checked={goster} onChange={olay => gosterAyarla(olay.target.checked)} />
  </label>;
}
