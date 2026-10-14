import type { Dil } from "../../../../ortak/dil";
import { metin } from "./metin";

interface Ozellikler {
  dil: Dil;
}


export function AltBilgi({ dil }: Ozellikler) {
  const m = metin[dil];
  return <footer className="foot">
    <span>{m.pasifDinleme}</span>
    <span>{m.surum}</span>
  </footer>;
}
