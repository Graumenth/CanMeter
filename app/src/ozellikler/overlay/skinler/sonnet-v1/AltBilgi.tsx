import type { Dil } from "../../../../ortak/dil";
import { metin } from "./metin";

interface AltBilgiOzellikleri {
  dil: Dil;
}

export function AltBilgi({ dil }: AltBilgiOzellikleri) {
  const m = metin[dil];
  return <footer className="foot">
    <span className="foot__live">
      <span className="dot" />
      {m.demoVerisi}
    </span>
    <span className="foot__ver">{m.surum}</span>
  </footer>;
}
