import { useSyncExternalStore } from "react";
import { dinle } from "../servis/motor";
import type { SavasAnlik } from "../tipler/savas";

let anlik: SavasAnlik | null = null;
const aboneler = new Set<() => void>();
const durdur = dinle(yeni => {
  anlik = yeni;
  aboneler.forEach(abonelik => abonelik());
});

const abone = (abonelik: () => void) => {
  aboneler.add(abonelik);
  return () => aboneler.delete(abonelik);
};
const anlikAl = () => anlik;

export function useSavas(): SavasAnlik | null {
  return useSyncExternalStore(abone, anlikAl, anlikAl);
}

void durdur;
