import { memo, useMemo } from "react";
import type { CSSProperties } from "react";
import { sure, yuzde } from "../../ortak/sayi";
import type { HedefAnlik } from "../../tipler/savas";

interface HedefCubuguOzellikleri {
  hedef: HedefAnlik;
}

export const HedefCubugu = memo(function HedefCubugu({ hedef }: HedefCubuguOzellikleri) {
  const hp = Math.max(0, Math.min(100, hedef.hp_yuzde));
  const kalan = hedef.kalan_sn_tahmin === null ? "Tahmin yok" : `${sure(hedef.kalan_sn_tahmin * 1000)} kaldı`;
  const dolguStili = useMemo<CSSProperties>(() => ({ width: `${hp}%` }), [hp]);

  return (
    <section className="hedef" aria-label="Hedef durumu">
      <div className="hedef-bilgi">
        <strong>{hedef.ad}</strong>
        <span>{yuzde(hp / 100)} · {kalan}</span>
      </div>
      <div className="hp-zemin"><i className="hp-dolgu" style={dolguStili} /></div>
    </section>
  );
});
