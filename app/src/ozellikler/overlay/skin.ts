import type { ComponentType } from "react";
import type { SavasAnlik } from "../../tipler/savas";
import type { Dil } from "../../ortak/dil";

/** Her skin'in aldığı veri ve kontroller. */
export interface SkinOzellikleri {
  savas: SavasAnlik | null;
  dil: Dil;
  kilitli: boolean;
  kilitDegistir: () => void;
  ayarlarAc: () => void;
  toplamGoster: boolean;
}

export interface Skin {
  ad: string;      // "gece" gibi, localStorage anahtarı
  etiket: string;  // kullanıcıya görünen ad
  Bilesen: ComponentType<SkinOzellikleri>;
}
