export type Sinif =
  | "Gladiator"
  | "Templar"
  | "Assassin"
  | "Ranger"
  | "Sorcerer"
  | "Spiritmaster"
  | "Cleric"
  | "Chanter"
  | "Bilinmiyor";

export interface HedefAnlik {
  ad: string;
  hp_yuzde: number;
  kalan_sn_tahmin: number | null;
}

export interface OyuncuAnlik {
  id: number;
  ad: string;
  sinif: Sinif;
  ben: boolean;
  olu: boolean;
  hasar: number;
  dps: number;
  pay: number;
  kritik_orani: number;
  iyilestirme: number;
  hps: number;
}

export interface SavasAnlik {
  savas_id: number;
  aktif: boolean;
  sure_ms: number;
  hedef: HedefAnlik | null;
  toplam_hasar: number;
  oyuncular: OyuncuAnlik[];
}
