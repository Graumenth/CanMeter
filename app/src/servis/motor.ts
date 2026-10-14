import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { SavasAnlik } from "../tipler/savas";

export function dinle(cb: (a: SavasAnlik) => void): () => void {
  let durdu = false;
  let kaldir: (() => void) | undefined;
  listen<SavasAnlik>("savas-anlik", ({ payload }) => cb(payload)).then(unlisten => {
    if (durdu) unlisten();
    else kaldir = unlisten;
  });
  return () => {
    durdu = true;
    kaldir?.();
  };
}

export const overlayKilit = (kilit: boolean) => invoke("overlay_kilit", { kilit });
export const surum = () => invoke<string>("surum");
