# Harita adları (`haritalar.toml`)

Oyunun harita yükleme mesajındaki (`21 36`) sayısal harita kimliğinin görünen adı. Topluluk verisi; sahip onayı
2026-10-19 (faz 02 görev 5). Adlar NC Corporation'ın oyun metinleridir; bu tablo yalnız bölgeyi tanıtmak için
kullanılır, CanMeter'ın GPL-3.0 lisansı kapsamında değildir.

| Kaynak | Alınan | Tarih |
|---|---|---|
| gamers4.life AION 2 veritabanı, "Dungeons" listesi, Map sütunu (https://gamers4.life/aion-2/database/en/dungeons/) | 548 kaydın görünen ilk 180'i (A-E arası adlar); zindan ve açık dünya alt bölgeleri | 2026-10-19 |
| cyberbadger6969/aion2-dps-meter (GPL-3.0; yalnız veri, kod alınmadı): `data/field_boss_maps.json` (1110 = Altgard) ve `data/dungeons/en.json` (parti zindanları 600001-600163, son basamak zorluk: 1 keşif, 2 normal, 3 zor) | 1110 ve 600xxx | 2026-10-19 |
| Sahip (oyun içi gözlem; uygulama bilinmeyen bölgede sorar, cevap `%APPDATA%\CanMeter\haritalar.toml`'a yazılır) | büyür | sürekli |

Doğrulama: 600032 kayıtlarda (2026-10-16 ve 17) parti zindanı olarak görüldü; gamers4 "Draupnir". 800012 ve 152002
açık dünya sunucusunda sunucu değişmeden yüklendi (yerel alt bölge). 1010, 100829, 500037, 500038 bulunamadı:
uygulama ilk girişte sorar.

# Beceri adları (`beceriler.toml`)

Hasar ve iyileştirme mesajlarındaki ham beceri kimliğinin (u32) İngilizce görünen adı; 9268 kayıt. Topluluk verisi;
sahip isteği 2026-10-29 ("IDleri internetten arayıp bulmayı deneyebilir misin"). Adlar NC Corporation'ın oyun
metinleridir; bu tablo yalnız beceriyi tanıtmak için kullanılır, CanMeter'ın GPL-3.0 lisansı kapsamında değildir.

| Kaynak | Alınan | Tarih |
|---|---|---|
| taengu/A2Tools-DPS-Meter (GPL-3.0; yalnız veri, kod alınmadı; commit 3bfb5eb, 2026-10-14): `src/data/i18n/skills/en.json` ("skill tables taken from the game data"); meter paket kimliğini bu tabloda dönüşümsüz arar | tüm kayıtlar, kimlik sırasıyla, `araclar` dışı tek seferlik betikle üretildi | 2026-10-29 |
| Çapraz kontrol: metabot.gg AION 2 veritabanı (oyun istemcisi veri tablolarından, build 0.0.4387.0) ikon yollarında aynı kimlik şemasını kullanıyor (ör. 11020000 Keen Strike, 11010000 Rending Blow) | yalnız doğrulama | 2026-10-29 |

Aranan ama ham kimlik vermeyen kaynaklar: metabot.gg, wikily.gg, aion2hub.com, gamewith.ai (adlar var, kimlik yok).
Türkçe tablo yok (kaynakta de/en/es/fr/ja/ko/pt/ru/zh var); şimdilik İngilizce. Kullanıcı dosyası
`%APPDATA%\CanMeter\beceriler.toml` yerleşiği ezer. Doğrulama: ilk gerçek savaş raporunda adların görünmesi
(kimlik şeması paketle aynıysa `#kimlik` kalmaz).
