import type { SavasAnlik } from "../tipler/savas";

// Tarayici demosu icin sabit savas verisi.

export const demoAnlik: SavasAnlik = {
  savas_id: 1,
  aktif: true,
  sure_ms: 61_000,
  hedef: { ad: "Ancient Guardian", hp_yuzde: 54, kalan_sn_tahmin: 70 },
  toplam_hasar: 28_600_000,
  oyuncular: [
    { id: 1, ad: "Ozan", sinif: "Gladiator", ben: false, olu: false,
      hasar: 6_750_000, dps: 113_000, pay: .236, kritik_orani: .28, iyilestirme: 0, hps: 0 },
    { id: 2, ad: "Kaelith", sinif: "Sorcerer", ben: true, olu: false,
      hasar: 6_720_000, dps: 112_000, pay: .235, kritik_orani: .38, iyilestirme: 0, hps: 0 },
    { id: 3, ad: "Nyra", sinif: "Assassin", ben: false, olu: false,
      hasar: 6_120_000, dps: 102_000, pay: .214, kritik_orani: .42, iyilestirme: 0, hps: 0 },
    { id: 4, ad: "Mira", sinif: "Chanter", ben: false, olu: false,
      hasar: 3_550_000, dps: 59_100, pay: .124, kritik_orani: .30, iyilestirme: 1_200_000, hps: 20_000 },
    { id: 5, ad: "Dorn", sinif: "Templar", ben: false, olu: true,
      hasar: 2_770_000, dps: 46_200, pay: .097, kritik_orani: .25, iyilestirme: 0, hps: 0 },
    { id: 6, ad: "Elara", sinif: "Cleric", ben: false, olu: false,
      hasar: 2_720_000, dps: 45_400, pay: .095, kritik_orani: .20, iyilestirme: 1_800_000, hps: 30_000 }
  ]
};
