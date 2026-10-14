import { bicimle } from "../../../../ortak/dil";
import type { CSSProperties } from "react";
import { sure, yuzdeTam } from "../../../../ortak/sayi";
import type { HedefAnlik } from "../../../../tipler/savas";
import { metin } from "./metin";
import type { SkinOzellikleri } from "../../skin";

export function Hedef({ hedef, dil }: { hedef: HedefAnlik | null; dil: SkinOzellikleri["dil"] }) {
  const m = metin[dil];
  if (!hedef) return null;
  const hp = Math.max(0, Math.min(100, hedef.hp_yuzde));
  const kalan = hedef.kalan_sn_tahmin == null ? null : bicimle(m.kaldi, sure(hedef.kalan_sn_tahmin * 1000));
  const stil = { "--hedef-hp": `${hp}%` } as CSSProperties;
  return <section className="target" aria-label={m.hedef}>
    <div className="target-info"><span className="target-icon">◇</span><h2 className="target-name">{hedef.ad}</h2>
      <span className="target-hp">{m.hp} {yuzdeTam(hp, dil)}</span>{kalan && <span className="target-time">{kalan}</span>}</div>
    <div className="hp-track" style={stil} role="meter" aria-label={m.hedefSagligi} aria-valuemin={0} aria-valuemax={100} aria-valuenow={hp}><i className="hp-fill" /></div>
  </section>;
}
