import type { Sinif } from "../../../../tipler/savas";

export function Logo() {
  return <svg className="logo" viewBox="0 0 20 20" aria-hidden="true"><path d="M10 1.6 17.7 6v8L10 18.4 2.3 14V6z"/><path d="M10 10l4.1-3.9"/><circle cx="10" cy="10" r="1.7"/></svg>;
}

export function SinifIkonu({ sinif }: { sinif: Sinif }) {
  const harfler: Record<Sinif, string> = { Gladiator: "G", Templar: "T", Assassin: "A", Ranger: "R", Sorcerer: "S", Spiritmaster: "M", Cleric: "C", Chanter: "H", Bilinmiyor: "?" };
  return <span className="sinif-ikon" aria-label={sinif}>{harfler[sinif]}</span>;
}

export function Kafatas() {
  return <svg viewBox="0 0 10 10" aria-hidden="true">
    <path d="M5 .8C2.8.8 1.2 2.3 1.2 4.3c0 1.3.6 2.2 1.5 2.8v1.9h4.6V7.1c.9-.6 1.5-1.5 1.5-2.8C8.8 2.3 7.2.8 5 .8z"/>
    <circle className="delik" cx="3.7" cy="4.3" r=".85"/><circle className="delik" cx="6.3" cy="4.3" r=".85"/>
  </svg>;
}
