import { memo } from "react";

const SURUM = "v0.0.3";
const DEMO_METNI = "Demo verisi";
const DURUM_SINIFI = "durum-noktasi";

export const AltBilgi = memo(function AltBilgi() {
  return (
    <footer className="alt-bilgi">
      <span>
        <i className={DURUM_SINIFI} /> {DEMO_METNI}
      </span>
      <span>{SURUM}</span>
    </footer>
  );
});
