import type { Sinif } from "../../../../tipler/savas";

export function Logo() {
  return <svg className="logo" width="15" height="15" viewBox="0 0 16 16" aria-hidden="true"><path d="M1.6 9h2.9l2-5.4 3 8.2 2-5.2h2.9" /></svg>;
}

export function HedefIkonu() {
  return <svg width="12" height="12" viewBox="0 0 12 12" aria-hidden="true"><path d="M1.6 3.9V2.5c0-.5.4-.9.9-.9h1.4M8.1 1.6h1.4c.5 0 .9.4.9.9v1.4M10.4 8.1v1.4c0 .5-.4.9-.9.9H8.1M3.9 10.4H2.5a.9.9 0 0 1-.9-.9V8.1" /><circle cx="6" cy="6" r="1.25" /></svg>;
}

export function SinifIkonu({ sinif }: { sinif: Sinif }) {
  if (sinif === "Gladiator") return <svg viewBox="0 0 12 12"><path d="m10.4 1.6-4.4 4.4M4.2 4.9l2.9 2.9M5.2 6.9 2.4 9.7" /></svg>;
  if (sinif === "Sorcerer") return <svg viewBox="0 0 12 12"><path d="m6 1.3 1.1 3.6L10.7 6 7.1 7.1 6 10.7 4.9 7.1 1.3 6l3.6-1.1Z" /></svg>;
  if (sinif === "Assassin") return <svg viewBox="0 0 12 12"><path d="M10.5 1.5C7.4 2.1 5.2 3.9 4.1 6.4l1.5 1.5c2.5-1.1 4.3-3.3 4.9-6.4Z" /></svg>;
  if (sinif === "Chanter") return <svg viewBox="0 0 12 12"><circle cx="6" cy="2.5" r="1.2" /><path d="M6 3.7v6.8m0-4.2h2.3M6 8.1H3.7" /></svg>;
  if (sinif === "Templar") return <svg viewBox="0 0 12 12"><path d="M6 1.5 9.9 2.9v3.2c0 2.4-1.7 3.7-3.9 4.4-2.2-.7-3.9-2-3.9-4.4V2.9Z" /></svg>;
  if (sinif === "Cleric") return <svg viewBox="0 0 12 12"><circle cx="6" cy="6" r="4.4" /><path d="M6 3.8v4.4M3.8 6h4.4" /></svg>;
  return <svg viewBox="0 0 12 12"><circle cx="6" cy="6" r="4" /><path d="M3.5 6h5M6 3.5v5" /></svg>;
}
