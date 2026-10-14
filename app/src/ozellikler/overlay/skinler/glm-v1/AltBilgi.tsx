import { memo } from "react";
import type { Dil } from "../../../../ortak/dil";
import { metin } from "./metin";

export const AltBilgi = memo(function AltBilgi({ dil }: { dil: Dil }) {
  const m = metin[dil];
  const surum = m.surum;
  return <footer className="foot">
    <span className="demo">
      <span className="dot" />
      {m.demoVerisi}
    </span>
    <span className="ver">{surum}</span>
  </footer>;
});
