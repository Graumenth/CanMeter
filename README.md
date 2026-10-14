# CanMeter

> AION 2 için pasif, ağ-dinleme tabanlı hasar ölçer (damage meter). Windows.

## Ne
Oyunun sunucudan gelen paketlerini Npcap ile okur, savaş olaylarını (hasar, iyileştirme, buff)
çözer ve canlı DPS/HPS, beceri kırılımı ve savaş geçmişi gösterir. Oyun istemcisine dokunmaz:
enjeksiyon yok, bellek okuma yok, sunucuya paket gönderme yok.

## Neden
Mevcut meter'lar ya kapalı kaynak ve imzasız (kime güvendiğin belli değil), ya da başka
dillerde ve bizim ihtiyacımıza göre değil. CanMeter kaynağı elimizde, Türkçe arayüzlü, sade.

## Yapı
- `crates/core`: Rust çekirdek (yakalama, protokol, savaş olayları, istatistik, replay).
- `app/`: Tauri v2 + React arayüz (faz 01'de eklenir).

Kurallar ve komutlar: `CLAUDE.md`. Kullanıcı-dönük değişiklikler: `PATCHNOTES.md`.

## Yasal
"AION" NCSOFT'un tescilli markasıdır; CanMeter NCSOFT ile ilişkili değildir. Üçüncü parti
araç kullanımı oyunun kurallarına aykırı olabilir; sorumluluk kullanıcıya aittir.
