import { kisalt } from "../../../../ortak/sayi";
import type { SavasAnlik } from "../../../../tipler/savas";
import type { SkinOzellikleri } from "../../skin";
import { metin } from "./metin";

interface Ozellikler { savas: SavasAnlik; dil: SkinOzellikleri["dil"]; toplamGoster?: boolean; }

export function AltBilgi({ savas, dil, toplamGoster }: Ozellikler) {
  const m = metin[dil];
  const dpsToplam = savas.oyuncular.reduce((toplam, oyuncu) => toplam + oyuncu.dps, 0);
  return <footer className="alt-bilgi"><span className="demo">{m.demoVerisi}</span>
    {toplamGoster && <span className="toplam">{m.toplamDps} {kisalt(dpsToplam)} · {m.hasar} {kisalt(savas.toplam_hasar)}</span>}
    <span className="surum">{m.surum}</span>
  </footer>;
}
