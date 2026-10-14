import type { HedefAnlik } from "../../../../tipler/savas";
import type { Dil } from "../../../../ortak/dil";
import { bicimle } from "../../../../ortak/dil";
import { sure, yuzdeTam } from "../../../../ortak/sayi";
import { metin } from "./metin";

export function Hedef({ hedef, oyuncuSayisi, dil }: { hedef: HedefAnlik | null; oyuncuSayisi: number; dil: Dil }) {
  const m = metin[dil];
  const hp = Math.max(0, Math.min(100, hedef?.hp_yuzde ?? 0));
  const kalan = hedef?.kalan_sn_tahmin == null ? null : sure(hedef.kalan_sn_tahmin * 1000);

  return <section className="target"><div className="t-top"><span className="lbl">{m.hedef}</span><b>{hedef?.ad ?? m.hedefYok}</b><strong><small>{m.hp}</small>{yuzdeTam(hp, dil)}</strong></div>
    <div className="hpbar"><i style={{ width: `${hp}%` }} /><span /></div><div className="t-bot"><span>{bicimle(m.oyuncular, oyuncuSayisi)}</span>{kalan && <b>{bicimle(m.kaldi, kalan)}</b>}</div>
  </section>;
}
