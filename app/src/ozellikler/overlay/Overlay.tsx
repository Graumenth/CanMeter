import { memo, useCallback, useState } from "react";
import type { CSSProperties } from "react";
import { useAyar } from "../../ortak/ayarlar";
import { useDil } from "../../ortak/dil";
import { useSavas } from "../../kancalar/useSavas";
import { overlayKilit } from "../../servis/motor";
import { Ayarlar } from "./ayarlar/Ayarlar";
import { useSkin } from "./skinler/useSkin";

export const Overlay = memo(function Overlay() {
  const savas = useSavas();
  const dil = useDil();
  const [skin] = useSkin();
  const [kilitli, kilitAyarla] = useState(false);
  const [ayarlarAcik, ayarlarAcikAyarla] = useState(false);
  const [opaklik] = useAyar("zeminOpaklik", 0.92);
  const [renk] = useAyar<string | null>("zeminRenk", null);
  const [toplamGoster] = useAyar("toplamGoster", true);
  const kilitDegistir = useCallback(() => {
    kilitAyarla(eski => {
      const yeni = !eski;
      void overlayKilit(yeni);
      return yeni;
    });
  }, []);
  const ayarlarAc = useCallback(() => ayarlarAcikAyarla(true), []);
  const ayarlarKapat = useCallback(() => ayarlarAcikAyarla(false), []);
  const stil = { "--zemin-opaklik": opaklik,
    ...(renk ? { "--zemin-renk": renk } : {}) } as CSSProperties;

  return <div className="overlay-kok" style={stil}>
    <skin.Bilesen savas={savas} dil={dil} kilitli={kilitli} kilitDegistir={kilitDegistir}
      ayarlarAc={ayarlarAc} toplamGoster={toplamGoster} />
    <Ayarlar acik={ayarlarAcik} kapat={ayarlarKapat} />
  </div>;
});
