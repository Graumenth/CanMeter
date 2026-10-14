# Overlay skin kaynakları

Her dosya tek başına açılan HTML+CSS mockup'tır (JavaScript yok, görsel yok). Sahip 2026-10-14'te iki
turluk tasarım yarışmasından bu altısını skin seçeneği olarak seçti. Skin'ler React'e
`app/src/ozellikler/overlay/skinler/<ad>/` altına taşınır. Adlar görünüme göre, modelden bağımsızdır (sahip kararı); mockup burada tasarım kaynağı olarak kalır.

| Skin | Kaynak | Durum |
|---|---|---|
| gece (Gece) | Claude Sonnet 5.5, 2. tur (özgün brief) | **varsayılan**, React'e taşındı |
| kor (Kor) | Claude Sonnet 5.5, 1. tur | React'e taşındı |
| sis (Sis) | Codex gpt-6-astra, 1. tur | React'e taşındı |
| obsidyen (Obsidyen) | Codex gpt-5.6-sol, 2. tur | React'e taşındı |
| kehribar (Kehribar) | GLM 5.3, 1. tur | React'e taşındı |
| komur (Kömür) | GLM 5.3, 2. tur | React'e taşındı |

Tur 1 brief'i: LOXIN benzeri, veri ve ölçü sabit. Tur 2 brief'i: "tamamen özgün ol, damage meter olsun",
yalnız veri verildi. Ortak veri: savaş 1:01, Ancient Guardian %54 / 1:10 kaldı, 6 oyuncu
(Ozan Gladiator 113K · Kaelith Sorcerer 112K ben · Nyra Assassin 102K · Mira Chanter 59.1K +20K HPS ·
Dorn Templar 46.2K ölü · Elara Cleric 45.4K +30K HPS).

Port kuralı: mockup'ın CSS'i olduğu gibi `skin.css`'e (kök sınıfla kapsanır), DOM'u küçük bileşenlere
(Overlay, Baslik, Hedef, Satir, AltBilgi, Toplam, ikonlar); metinler `metin.ts` (tr/en); veri `SavasAnlik`'ten;
her skin'de kilit ve dişli düğmesi, `data-tauri-drag-region`, yalnız yazı "Toplam DPS · Hasar" göstergesi;
panel zemini `color-mix(var(--zemin-renk), var(--zemin-opaklik))` ile ayarlardan renk/şeffaflık alır.
Geliştirme: `npm run dev` ile tarayıcıda `?skin=<ad>&dil=en` sabit demo veriyle açılır.
