import { memo } from "react";
import type { HedefAnlik } from "../../../../tipler/savas";
import { bicimle, type Dil } from "../../../../ortak/dil";
import { sure, yuzdeTam } from "../../../../ortak/sayi";
import { HedefIkonu } from "./ikonlar";
import { metin } from "./metin";

export const Hedef = memo(function Hedef({ hedef, dil }: { hedef: HedefAnlik | null; dil: Dil }) {
  const m = metin[dil];
  const hp = Math.max(0, Math.min(100, hedef?.hp_yuzde ?? 0));
  const kalan = hedef?.kalan_sn_tahmin == null ? null : bicimle(m.kaldi, sure(hedef.kalan_sn_tahmin * 1000));
  return <section className="target"><div className="target-line"><span className="target-name"><HedefIkonu />{hedef?.ad ?? m.hedefYok}</span>
    <span className="target-meta"><span className="t-hp">{m.hp} {yuzdeTam(hp, dil)}</span>{kalan && <><span className="t-sep" /><span className="t-time">{kalan}</span></>}</span></div>
    <div className="hp-track"><div className="hp-fill" style={{ width: `${hp}%` }} /></div>
  </section>;
});
