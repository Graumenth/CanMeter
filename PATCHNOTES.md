# PATCHNOTES

## 0.3.4 (2026-10-14)
### Düzeltmeler
- Overlay kilitliyken artık Ctrl+Shift+K ile kilidi açabilirsiniz; kısayol oyun açıkken de çalışır.
  Kısayollar ayarlar panelinde listelenir.

## 0.3.3 (2026-10-14)
### Değişti
- İlk açılışta arayüz dili bilgisayarınızın diline göre seçilir: Türkçe sistemde Türkçe, diğer dillerde
  İngilizce. Ayarlardan ya da Ctrl+Shift+L ile değiştirebilirsiniz.

## 0.3.2 (2026-10-14)
### Düzeltmeler
- Kömür görünümünde hasar çubukları artık satır boyunca uzuyor ve hasara göre değişiyor.

## 0.3.1 (2026-10-14)
### Düzeltmeler
- Kehribar görünümünde oyuncu çubukları artık hasara göre uzayıp kısalıyor; satırdaki renk tonu yumuşak geçişli.

## 0.3.0 (2026-10-14)
### Yeni
- Overlay'i kenarından tutup büyütüp küçültebilirsiniz; yazılar ve çubuklar dahil tüm görünüm orantılı
  ölçeklenir, yükseklik içeriğe göre kendiliğinden ayarlanır. Seçtiğiniz boyut hatırlanır.

## 0.2.5 (2026-10-14)
### Değişti
- Overlay artık tarayıcı bileşeni (WebView2) kullanmıyor; tek bir küçük program olarak çalışır ve çok daha az
  bellek tutar. Görünümler, ayarlar ve dil seçeneği aynen korundu.
- Ayarlar artık `%APPDATA%\CanMeter\ayarlar.toml` dosyasında saklanır; eski sürümün ayarları taşınmaz.

## 0.2.4 (2026-10-14)
### Değişti
- Sürüm numarası 0.2.4: görünümler ve ayarlar paketi ile ilk overlay ayrı yetenekler olarak sayıldı.
  Davranış değişikliği yok.

## 0.1.0 (2026-10-14)
### Değişti
- Sürüm 0.1.0: 0.0.7'den bu yana gelen görünümler, ayarlar paneli ve dil seçeneği yeni yetenekler
  olduğu için sürüm basamağı düzeltildi. Davranış değişikliği yok.

## 0.0.12 (2026-10-14)
### Düzeltmeler
- Overlay boşta çok daha az işlemci kullanır: bar animasyonları kaldırıldı (işlemci payı yaklaşık sekizde bire indi).

## 0.0.11 (2026-10-14)
### Değişti
- Görünüm adları: Gece, Kor, Sis, Obsidyen, Kehribar, Kömür (önce model adları).
- Her görünümün üstünde büyük rakamlarla toplam DPS (solda) ve toplam hasar (sağda); Gece'deki parti
  payı şeridi kaldırıldı.

## 0.0.10 (2026-10-14)
### Yeni
- Altı farklı overlay görünümü (skin); ayarlardan ya da Ctrl+Shift+S ile değişir. Varsayılan: Sonnet v2
  (parti pay şeridi, büyük DPS rakamları, sınıf ikonları).
- Ayarlar paneli: dil (Türkçe / English), görünüm, panel şeffaflığı, panel zemin rengi, toplam göstergesi.
- Her görünümde yazı olarak "Toplam DPS · Hasar" göstergesi.
- İngilizce arayüz seçeneği (ayarlardan ya da Ctrl+Shift+L); varsayılan Türkçe.

## 0.0.9 (2026-10-14)
### Değişti
- Görünürde değişiklik yok; proje yönerge dosyası depo dışına alındı (iç düzen).

## 0.0.8 (2026-10-14)
### Düzeltmeler
- `kaydet` aracı kayıtları her zaman proje kökündeki `kayitlar/` klasörüne yazar; nereden
  çalıştırıldığı fark etmez.

## 0.0.7 (2026-10-14)
### Yeni
- Overlay: savaş süresi, hedef HP ve kalan süre, parti DPS sıralaması, sınıf renkli barlar, kendi
  satırının vurgusu, ölü oyuncu işareti, kilit düğmesi (kilitliyken tıklamalar oyuna geçer).

## 0.0.6 (2026-10-14)
### Yeni
- Masaüstü uygulama kabuğu: şeffaf, çerçevesiz, hep üstte kalan overlay penceresi. Çekirdekten
  saniyede en çok 4 kez, yalnız değişiklik varsa veri alır (düşük işlemci kullanımı).

## 0.0.5 (2026-10-14)
### Yeni
- Çekirdek, arayüze savaş durumunu (oyuncular, DPS, pay, hedef HP ve kalan süre) tek bir
  sözleşmeyle verir. Oyuna girmeden arayüz geliştirmek için demo savaş üreteci eklendi.

## 0.0.4 (2026-10-14)
### Değişti
- README'nin tanıtım metni (Ne / Neden) daha doğal bir dille yeniden yazıldı; içerik aynı.

## 0.0.3 (2026-10-14)
### Yeni
- Geliştirici aracı `kaydet`: seçilen adaptördeki TCP trafiğini `kayitlar/<tarih>.pcap` dosyasına
  kaydeder; her 10 saniyede en yoğun akışları, bitişte "oyun sunucusu adayı"nı (80/443 dışı,
  en çok veri gelen uç) yazar. Ctrl+C ile temiz kapanır, `--sure` ile süreli kayıt.
  Hiçbir paket göndermez. Oyuncu adı içerebileceği için `kayitlar/` git dışıdır.

## 0.0.2 (2026-10-14)
### Yeni
- Ağ adaptörü seçimi: Npcap'in gördüğü adaptörler listelenir, internete çıkan adaptör
  kendiliğinden seçilir; istenirse sıra numarası, ad ya da açıklama parçasıyla elle seçilir.
  Bağlı olmayan Wi-Fi/Bluetooth adaptörleri (yalnız 169.254.x.x adresi olanlar) elenir.
- Geliştirici: `cargo run -p canmeter-core --example arayuzler` adaptörleri ve seçimi gösterir.
  Npcap SDK `<DerlemeAraclari>\npcap-sdk` altından derleme betiğiyle bulunur (`NPCAP_SDK` ile ezilebilir).

## 0.0.1 (2026-10-14)
### Yeni
- Proje iskeleti: çekirdek kütüphane (`canmeter-core`), kurallar, koordinasyon klasörü.
  Henüz hiçbir ölçüm yapmıyor; ilk çalışan sürüm faz 01 sonunda gelecek.
