import { bicimle } from "../../../../ortak/dil";
import { sure, yuzdeTam } from "../../../../ortak/sayi";
import type { HedefAnlik } from "../../../../tipler/savas";
import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";

interface Ozellikler { hedef: HedefAnlik | null; dil: SkinOzellikleri["dil"]; }

export function Hedef({ hedef, dil }: Ozellikler) {
  const m = metin[dil];
  const hp = Math.max(0, Math.min(100, hedef?.hp_yuzde ?? 0));
  const stil = { width: `${hp}%` };
  return <>
    <div className="hedef-satiri"><h1>{hedef?.ad ?? m.hedefYok}</h1>
      <div className="hp-sayi"><small>{m.hp}</small>{yuzdeTam(hp, dil)}</div></div>
    <div className="boss-serit" aria-label={`${hedef?.ad ?? m.hedefYok} ${yuzdeTam(hp, dil)}`}><span style={stil} /></div>
    <div className="hedef-meta"><span>{m.hedefAnalizi}</span><span>{hedef?.kalan_sn_tahmin == null ? "" : bicimle(m.kaldi, sure(hedef.kalan_sn_tahmin * 1000))}</span></div>
  </>;
}
