# CanMeter

> AION 2 için pasif, ağ-dinleme tabanlı hasar ölçer (damage meter). Windows.

## Ne
CanMeter, AION 2 sunucusundan gelen paketleri Windows'ta Npcap ile pasif olarak dinler.
Bu paketlerdeki hasar ve iyileştirme olaylarını, ayrıca buff bilgilerini çözer.
Canlı DPS/HPS değerlerini ve beceri kırılımını gösterir, savaş geçmişini tutar.
Oyun istemcisine enjeksiyon yapmaz ve belleğini okumaz. Sunucuya paket göndermez.

## Neden
Mevcut meter'lar ya kapalı kaynak ve imzasız, ya da bizim ihtiyacımıza göre değil.
CanMeter'da kaynak kodu elimizde, gerektiğinde kendimiz değişiklik yapabiliyoruz.
Arayüzü Türkçe yaptık. İhtiyacımız olan bilgileri gösteren sade bir meter olsun istedik.

## Yapı
- `crates/core`: Rust çekirdek (yakalama, protokol, savaş olayları, istatistik, replay).
- `crates/app`: overlay arayüzü, saf Rust (egui). Tek süreç, tarayıcı bileşeni yok.

Kurallar ve komutlar: `CLAUDE.md`. Kullanıcı-dönük değişiklikler: `PATCHNOTES.md`.

## Yasal
"AION" NCSOFT'un tescilli markasıdır; CanMeter NCSOFT ile ilişkili değildir. Üçüncü parti
araç kullanımı oyunun kurallarına aykırı olabilir; sorumluluk kullanıcıya aittir.
