# CLAUDE.md - CanMeter

> **Genel kurallar:** `\Projeler\!KURALLAR\GENEL-KURALLAR.md` bu projede aynen geçerlidir
> (sürümleme, changelog, sırlar, git akışı, §6.3 mühendislik disiplini, §6.4 modüler yapı,
> §14.3 delegasyon). Bu dosya yalnız CanMeter'a özgü kararları taşır; genel kural burada tekrarlanmaz.

## Proje
**CanMeter**: AION 2 için pasif ağ-dinleme tabanlı hasar/iyileştirme ölçer (damage meter), Windows.
Oyun istemcisine dokunmaz: enjeksiyon yok, bellek okuma yok, sunucuya tek paket gönderilmez;
yalnız Npcap ile gelen paketler okunur. Marka adı **CanMeter**; "AION" yalnız açıklamada geçer
(tescil NCSOFT'un), ürün adında kullanılmaz.

## Stack (karar)
- **Rust çekirdek** `crates/core` (`canmeter-core`): yakalama, TCP birleştirme, protokol çözümü,
  savaş olayları, toplama (DPS/HPS). Saf kütüphane, UI'ı hiç görmez.
- **Tauri v2 + React/TypeScript** uygulama kabuğu `app/`: overlay, geçmiş, ayarlar. Yalnız
  `core`'un açık API'sini görür (GENEL §6.4, tek yönlü bağımlılık).
- Paket yakalama: `pcap` crate + **Npcap**. Npcap kullanıcı tarafından kurulur, pakete
  gömülmez (OEM lisansı yok). Link için Npcap SDK `<DerlemeAraclari>\npcap-sdk`.
- Araçlar: Rust **1.98.1** (`rust-toolchain.toml`), Node 24, npm (pnpm kullanılmaz), tauri-cli
  projeye `devDependency` olarak gelir (global kurulum yok). WebView2 runtime sistemde.
- **Köken/lisans (sahip onayı bekliyor):** kod bizim, kapalı kalabilir. GPL-3.0 açık kaynak
  meter'lar (A2Tools, NOIA2, RATmeter) yalnız **protokol referansı**; onlardan kod kopyalanmaz.

## Mimari
- `crates/core/src/capture/`: arayüz seçimi, BPF filtresi, TCP akış birleştirme.
- `crates/core/src/protocol/`: çerçeveleme, varint, sıkıştırma, opcode tabloları. **Test-önce** (GENEL §6.3).
- `crates/core/src/combat/`: olay modeli (hasar, iyileştirme, buff), entity-isim eşleme, summon birleştirme.
- `crates/core/src/stats/`: savaş penceresi, DPS/HPS, beceri kırılımı.
- `crates/core/src/replay/`: pcap dosyasından çevrimdışı oynatma (oyuna girmeden geliştirme ve test).
- `app/`: Tauri kabuğu + React UI (faz 01'de üretilir, workspace üyesi `app/src-tauri`).
- Pcap kayıtları `kayitlar/`: git ve senkron dışı; oyuncu adı içerir, sır gibi muamele görür.

## Komutlar
- Çekirdek test: `cargo test --workspace` (repo kökü).
- Lint: `cargo clippy --workspace` · biçim: `cargo fmt --all`.
- Derleme çıktısı adı (GENEL §1.1): `CanMeter-<sürüm>-Kurulum.exe`; sürüm kaynak dosyadan
  script ile okunur, elle yazılmaz. App komutları Tauri iskeleti kurulunca buraya eklenir.

## Güvenlik ve ToS
- Geliştirme ve test **ikinci (test) hesapla** yapılır; NCSOFT üçüncü parti programı yasaklar, ban riski gerçektir.
- Ürün hiçbir zaman: enjeksiyon, bellek okuma, paket gönderme, otomasyon yapmaz. Bu satır mimari sınırdır.

## Durum / koordinasyon
Canlı durum `CanMeterCC\STATE.md`. Plan kartları `CanMeterCC\phases\`. Araştırma `CanMeterCC\arastirma-*.md`.
Worklog/backlog CC'de (GENEL §10). Sürüm ve kullanıcı-dönük notlar `PATCHNOTES.md` (GENEL §1-2).
