import { kisalt } from "../../../../ortak/sayi";
import type { SavasAnlik } from "../../../../tipler/savas";
import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";

export function Toplam({ savas, dil }: { savas: SavasAnlik; dil: SkinOzellikleri["dil"] }) {
  const m = metin[dil];
  const dpsToplam = savas.oyuncular.reduce((toplam, oyuncu) => toplam + oyuncu.dps, 0);
  return <span className="toplam num">{m.toplamDps} {kisalt(dpsToplam)} · {m.hasar} {kisalt(savas.toplam_hasar)}</span>;
}
