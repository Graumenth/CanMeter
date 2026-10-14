import type { Sinif } from "../../../../tipler/savas";

interface SinifIkonuOzellikleri {
  sinif: Sinif;
}

export function SinifIkonu({ sinif }: SinifIkonuOzellikleri) {
  const ortak = { viewBox: "0 0 16 16", "aria-hidden": true };
  if (sinif === "Gladiator") return <svg {...ortak}><path d="M8 1l6 4v6l-6 4-6-4V5zM8 5v6M5 8h6" /></svg>;
  if (sinif === "Templar") return <svg {...ortak}><path d="M8 1l5 2v4c0 3-2 5-5 7-3-2-5-4-5-7V3z" /></svg>;
  if (sinif === "Assassin") return <svg {...ortak}><path d="M3 13L13 3m0 0H8m5 0v5M3 9l4 4" /></svg>;
  if (sinif === "Ranger") return <svg {...ortak}><path d="M3 13L13 3M4 4l8 8M3 10l3 3M10 3l3 3" /></svg>;
  if (sinif === "Sorcerer") return <svg {...ortak}><circle cx="8" cy="8" r="5.5" /><path d="M8 4l1.3 2.7L12 8l-2.7 1.3L8 12l-1.3-2.7L4 8l2.7-1.3z" /></svg>;
  if (sinif === "Spiritmaster") return <svg {...ortak}><circle cx="8" cy="8" r="5.5" /><path d="M8 3v10M3 8h10" /></svg>;
  if (sinif === "Cleric") return <svg {...ortak}><path d="M6.5 2h3v3.5H13v3H9.5V12h-3V8.5H3v-3h3.5z" /></svg>;
  if (sinif === "Chanter") return <svg {...ortak}><path d="M3 8c2-4 3-4 5 0s3 4 5 0M8 11v2" /><circle cx="8" cy="14" r="1" /></svg>;
  return <svg {...ortak}>
    <circle cx="8" cy="8" r="5.5" />
    <path d="M5 8h6M8 5v6" />
  </svg>;
}
