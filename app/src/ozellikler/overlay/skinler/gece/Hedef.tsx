import { sure, yuzdeTam } from "../../../../ortak/sayi";
import type { SavasAnlik } from "../../../../tipler/savas";
import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";

export function Hedef({ hedef, dil }: { hedef: SavasAnlik["hedef"]; dil: SkinOzellikleri["dil"] }) {
  const m = metin[dil];
  const hp = Math.max(0, Math.min(100, hedef?.hp_yuzde ?? 0));
  return <>
    <div className="target">
      <div className="row">
        <span className="name">{hedef?.ad ?? m.hedefYok}</span>
        {hedef?.kalan_sn_tahmin != null && <span className="eta">{m.kalan} <b className="num">~{sure(Math.round(hedef.kalan_sn_tahmin * 1000))}</b></span>}
      </div>
      <div className="hp"><i style={{ width: `${hp}%` }} />
        {[25, 50, 75].map(deger => <u key={deger} style={{ left: `${deger}%` }} />)}
      </div>
    </div>
    <div className="hpv num">{yuzdeTam(hp, dil)}</div>
  </>;
}
