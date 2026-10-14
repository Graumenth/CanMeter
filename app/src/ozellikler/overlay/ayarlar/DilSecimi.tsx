import { dilSec, diller, useDil } from "../../../ortak/dil";
import { metin } from "./metin";

export function DilSecimi() {
  const dil = useDil();
  return <div className="ayar-satiri">
    <span>{metin[dil].dil}</span>
    <div className="secenekler">
      {diller.map(secenek => <button key={secenek} type="button"
        className={secenek === dil ? "etkin" : ""}
        aria-pressed={secenek === dil} onClick={() => dilSec(secenek)}>
        {secenek.toUpperCase()}
      </button>)}
    </div>
  </div>;
}
