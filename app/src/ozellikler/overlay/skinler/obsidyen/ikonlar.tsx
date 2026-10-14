import type { Sinif } from "../../../../tipler/savas";

const ikonlar: Record<Sinif, string> = {
  Gladiator: "⚔", Templar: "⬡", Assassin: "◈", Ranger: "⌁", Sorcerer: "✦",
  Spiritmaster: "✧", Cleric: "✚", Chanter: "◌", Bilinmiyor: "?",
};

const kisaltmalar: Record<Sinif, string> = {
  Gladiator: "GLA", Templar: "TEM", Assassin: "ASS", Ranger: "RAN", Sorcerer: "SOR",
  Spiritmaster: "SPI", Cleric: "CLE", Chanter: "CHA", Bilinmiyor: "?",
};

export function SinifIkonu({ sinif }: { sinif: Sinif }) {
  return <span className="sinif-ikonu" title={sinif} aria-label={sinif}>{ikonlar[sinif]}</span>;
}

export function SinifKisaltmasi({ sinif }: { sinif: Sinif }) {
  return <span className="sinif-kisaltma">{kisaltmalar[sinif]}</span>;
}
