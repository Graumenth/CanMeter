import type { CSSProperties } from "react";
import type { Dil } from "../../../../ortak/dil";
import { bicimle } from "../../../../ortak/dil";
import type { HedefAnlik } from "../../../../tipler/savas";
import { sure, yuzdeTam } from "../../../../ortak/sayi";
import { metin } from "./metin";

const hpStili = (hp: number) => ({ "--hp": `${Math.max(0, Math.min(100, hp))}%` }) as CSSProperties;

export function Hedef({ hedef, dil }: { hedef: HedefAnlik; dil: Dil }) {
  const m = metin[dil];
  const kalan = hedef.kalan_sn_tahmin == null ? null : bicimle(m.kaldi, sure(hedef.kalan_sn_tahmin * 1000));
  return <section className="target"><div className="target__line">
    <span className="target__name">{hedef.ad}</span><span className="target__hp">{yuzdeTam(hedef.hp_yuzde, dil)}</span>
    {kalan && <span className="target__eta">{kalan}</span>}
  </div><div className="hpbar"><div className="hpbar__fill" style={hpStili(hedef.hp_yuzde)} /></div></section>;
}
