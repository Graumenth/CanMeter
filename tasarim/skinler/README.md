# Overlay skin kaynakları

Her dosya tek başına açılan HTML+CSS mockup'tır (JavaScript yok, görsel yok). Sahip 2026-10-14'te iki
turluk tasarım yarışmasından bu altısını skin seçeneği olarak seçti. Skin'ler egui'ye
`crates/app/src/skin/<ad>/` altına taşındı (önce React'e, o port `webview2` dalında durur). Adlar görünüme göre,
modelden bağımsızdır (sahip kararı); mockup burada tasarım kaynağı olarak kalır.

| Skin | Kaynak | Durum |
|---|---|---|
| gece (Gece) | Claude Sonnet 5.5, 2. tur (özgün brief) | **varsayılan**, egui'de |
| kor (Kor) | Claude Sonnet 5.5, 1. tur | egui'de |
| sis (Sis) | Codex gpt-6-astra, 1. tur | egui'de |
| obsidyen (Obsidyen) | Codex gpt-5.6-sol, 2. tur | egui'de |
| kehribar (Kehribar) | GLM 5.3, 1. tur | egui'de |
| komur (Kömür) | GLM 5.3, 2. tur | egui'de |

Tur 1 brief'i: LOXIN benzeri, veri ve ölçü sabit. Tur 2 brief'i: "tamamen özgün ol, damage meter olsun",
yalnız veri verildi. Ortak veri: savaş 1:01, Ancient Guardian %54 / 1:10 kaldı, 6 oyuncu
(Ozan Gladiator 113K · Kaelith Sorcerer 112K ben · Nyra Assassin 102K · Mira Chanter 59.1K +20K HPS ·
Dorn Templar 46.2K ölü · Elara Cleric 45.4K +30K HPS).

Port kuralı (egui): sözleşme `crates/app/src/skin/mod.rs` (donmuş); mockup'ın renkleri `renk.rs`, metinleri
`metin.rs` (tr/en), parçaları `baslik · hedef · satir · toplam · alt_bilgi · ikon` dosyaları; her bölüm kendi
yüksekliğini ayırır, çizim painter ile (zemin -> çubuk -> metin); animasyon yok; panel zemini
`b.zemin.dolgu(self.zemin_rengi())` ile ayarlardan renk/şeffaflık alır; her skin'de dişli ve kilit düğmesi,
başlıkta sürükleme, yalnız yazı "Toplam DPS · Hasar" göstergesi.
Doğrulama: `cargo run --release -p canmeter-app -- --skin <ad> --dil en`.
