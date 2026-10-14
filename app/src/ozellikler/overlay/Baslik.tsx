import { memo } from "react";

interface BaslikOzellikleri {
  kilitli: boolean;
  sure: string;
  kilitDegistir: () => void;
}

export const Baslik = memo(function Baslik({ kilitli, sure, kilitDegistir }: BaslikOzellikleri) {
  const kilitEtiketi = kilitli ? "Kilidi aç" : "Kilitle";

  return (
    <header className="baslik" data-tauri-drag-region>
      <strong className="marka" data-tauri-drag-region>CanMeter</strong>
      <time className="savas-suresi" data-tauri-drag-region>{sure}</time>
      <span className="mod" data-tauri-drag-region>DPS</span>
      <button className="kilit-dugmesi" type="button" aria-label={kilitEtiketi} onClick={kilitDegistir}>
        {kilitli ? "▣" : "▢"}
      </button>
    </header>
  );
});
