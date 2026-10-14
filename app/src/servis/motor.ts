import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { SavasAnlik } from "../tipler/savas";
import { demoAnlik } from "./demoAnlik";

const tauriVar = "__TAURI_INTERNALS__" in window;

export function dinle(cb: (a: SavasAnlik) => void): () => void {
  if (!tauriVar) { setTimeout(() => cb(demoAnlik), 0); return () => {}; }
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

export const overlayKilit = (kilit: boolean) => tauriVar ? invoke("overlay_kilit", { kilit }) : Promise.resolve();
export const surum = () => tauriVar ? invoke<string>("surum") : Promise.resolve("dev");
