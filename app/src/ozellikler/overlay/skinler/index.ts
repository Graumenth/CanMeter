import type { Skin } from "../skin";
import { Gece } from "./gece/Overlay";
import { Kor } from "./kor/Overlay";
import { Sis } from "./sis/Overlay";
import { Obsidyen } from "./obsidyen/Overlay";
import { Kehribar } from "./kehribar/Overlay";
import { Komur } from "./komur/Overlay";

// Sahip seçimi (2026-10-14): altı skin; kaynak mockup'lar app/tasarim/skinler/.
export const skinler: Skin[] = [
  { ad: "gece", etiket: "Gece", Bilesen: Gece },
  { ad: "kor", etiket: "Kor", Bilesen: Kor },
  { ad: "sis", etiket: "Sis", Bilesen: Sis },
  { ad: "obsidyen", etiket: "Obsidyen", Bilesen: Obsidyen },
  { ad: "kehribar", etiket: "Kehribar", Bilesen: Kehribar },
  { ad: "komur", etiket: "Kömür", Bilesen: Komur },
];
export const varsayilanSkin = "gece";
