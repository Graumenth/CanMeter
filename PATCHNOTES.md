# PATCHNOTES

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
