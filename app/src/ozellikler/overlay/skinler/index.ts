import type { Skin } from "../skin";
import { SonnetV2 } from "./sonnet-v2/Overlay";
import { SonnetV1 } from "./sonnet-v1/Overlay";
import { AstraV1 } from "./astra-v1/Overlay";
import { SolV2 } from "./sol-v2/Overlay";
import { GlmV1 } from "./glm-v1/Overlay";
import { GlmV2 } from "./glm-v2/Overlay";

// Sahip seçimi (2026-10-14): altı skin; kaynak mockup'lar app/tasarim/skinler/.
export const skinler: Skin[] = [
  { ad: "sonnet-v2", etiket: "Sonnet v2", Bilesen: SonnetV2 },
  { ad: "sonnet-v1", etiket: "Sonnet v1", Bilesen: SonnetV1 },
  { ad: "astra-v1", etiket: "Astra v1", Bilesen: AstraV1 },
  { ad: "sol-v2", etiket: "Sol v2", Bilesen: SolV2 },
  { ad: "glm-v1", etiket: "GLM v1", Bilesen: GlmV1 },
  { ad: "glm-v2", etiket: "GLM v2", Bilesen: GlmV2 },
];
export const varsayilanSkin = "sonnet-v2";
