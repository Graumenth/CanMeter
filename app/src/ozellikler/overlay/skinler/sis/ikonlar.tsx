import type { Sinif } from "../../../../tipler/savas";

const yollar: Partial<Record<Sinif, string>> = {
  Gladiator: "m5 3 4 2 10 14m0-16-4 2L5 19M3 16l5 4m8 0 5-4M4 21l2-3m12 0 2 3",
  Sorcerer: "m12 2 2.5 6.5L21 11l-6.5 2.5L12 20l-2.5-6.5L3 11l6.5-2.5ZM19 3v3m-1.5-1.5h3M5 18v3m-1.5-1.5h3",
  Assassin: "m15 3-8 9 4 3 4-12Zm-9 8 7 6m-4-3-5 6m14-8-4 5 3 2 1-7Zm-4 5 5 3m-3-2-2 4",
  Chanter: "M12 10v12M8 3l4-2 4 2v5l-4 3-4-3ZM5 9c0 4 3 6 7 6s7-2 7-6M9 19h6",
  Templar: "m12 2 8 3v6c0 5-8 10-8 10S4 16 4 11V5l8-3Zm0 4v11M8 10h8",
  Cleric: "M10 3h4v6h6v4h-6v8h-4v-8H4V9h6ZM3 3l2 2m14 0 2-2",
  Ranger: "m4 20 16-16M7 4h13v13M5 9l2-5 5 2M15 18l5-1-1-5",
  Spiritmaster: "M12 3a7 7 0 1 0 7 7M12 7a3 3 0 1 0 3 3m-3 3v8m-4-3 4 3 4-3",
};

export function SinifIkonu({ sinif }: { sinif: Sinif }) {
  return <svg viewBox="0 0 24 24" aria-hidden="true"><path d={yollar[sinif] ?? "M12 3a9 9 0 1 0 9 9 9 9 0 0 0-9-9Zm-4 9h8M12 8v8"} /></svg>;
}

export function MarkaIkonu() {
  return <svg className="brand-mark" viewBox="0 0 24 24" aria-hidden="true"><path d="M17.5 4.5 12 2 4 6.5v10l8 5.5 5.5-3.5" /><path d="m8 14 3-5 3 7 3-5h4" strokeWidth="2" /></svg>;
}
