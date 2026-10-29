# PATCHNOTES

## 0.8.10 (2026-10-29)
### Düzeltmeler
- Savaş artık yalnız sizin ya da partinizin verdiği veya aldığı hasarla açılıyor ve sürüyor. Çevredeki yabancı
  oyuncuların vuruşları savaşı başlatmıyor ve uzatmıyor; kalabalık bölgede savaş dakikalarca açık kalmıyor, kendini
  yenileme de gecikmiyor. Savaş açıkken hedefinize vuran yabancılar yine satır alır.

## 0.8.9 (2026-10-29)
### Yeni
- Beceri adları: raporda beceriler artık numara yerine adıyla görünüyor (9268 beceri, İngilizce; topluluk verisi).
  Bilinmeyen beceri yine #numara. Kendi adlarınızı %APPDATA%\CanMeter\beceriler.toml dosyasına yazabilirsiniz.

## 0.8.8 (2026-10-29)
### Düzeltmeler
- CanMeter artık ikinci kez açılmıyor: zaten çalışıyorsa yeni kopya sessizce kapanır. İki overlay ve çift sayım
  olmuyor. Yeni sürüme kendini yenilerken eski kopya kilidi bırakır, yeni kopya onu bekler.

## 0.8.7 (2026-10-29)
### Yeni
- Ayarlar panelinde "Hub'ı aç" düğmesi: CanMeter Hub kuruluysa (meter klasörünün yanındaki CanMeter Hub klasörü)
  tek tıkla açılır; kurulu değilse düğme kapalı durur ve nedenini söyler. Hub programı yakında.

## 0.8.6 (2026-10-29)
### Değişti
- CanMeter artık kurulu bir program gibi sabit yerden çalışır: D:\CanMeter\Meter (meter, bekçi, rapor). Derleme
  betiği her yeni sürümü oraya da yazar; çalışan meter yeni sürümü görünce savaş dışında kendini yeniler. Ayarlar ve
  geçmiş dosyaları yerinde kaldı.

## 0.8.5 (2026-10-29)
### Değişti
- Görünürde değişiklik yok: CanMeter artık Rust 1.99.1 ile derleniyor (önce 1.98.1).

## 0.8.4 (2026-10-29)
### Yeni
- Oyunla başlat: küçük bir bekçi programı (canmeter-bekci.exe) oyun açılınca CanMeter'ı kendiliğinden açar, oyun
  kapanınca dokunmaz. Kurulum tek komut: araclar\oyunla-baslat-kur.ps1 (yönetici PowerShell); kaldırmak için -Kaldir.
  Oyuna dokunmaz, yalnız çalışan programların listesine bakar; boşta 1 MB'tan az bellek.

## 0.8.3 (2026-10-19)
### Düzeltme
- Rapor aracı henüz kayıtlı savaş yokken hata vermiyor; boş rapor açılır ve savaşların nerede birikeceğini söyler.

## 0.8.2 (2026-10-19)
### Düzeltme
- Oyun açıkken CanMeter'ı başlatıp zindana girince ilk saniyeler (bölge, karakter, parti listesi) kaçabiliyordu;
  artık yeni sunucu bağlantısı ilk paketinden itibaren tamponlanır.

## 0.8.1 (2026-10-19)
### Yeni
- Savaşlar artık girilen bölgeyle (zindan, açık dünya alanı) kaydediliyor: rapor sol şeritte koşuya göre gruplar,
  index'te "Zindanlar" ve "Bölgeler" özetleri, bölge filtresi. Girilen her bölge savaş olmasa da kaydedilir.
- 222 bölge adı yerleşik. Bilinmeyen bir bölgeye girince CanMeter adını sorar (overlay'in altındaki satır; kilit
  açıkken yazılır); cevap `%APPDATA%\CanMeter\haritalar.toml` dosyasına kalıcı yazılır ve eski kayıtlar da o adla
  görünür.

## 0.8.0 (2026-10-19)
### Yeni
- Biten savaşlar artık diske kaydediliyor (`%LOCALAPPDATA%\CanMeter\encounters`): oyun oynarken dosyalar kendiliğinden
  birikir, `canmeter-rapor` çift tıklamayla gerçek savaşlarınızı gösterir (liste, DPS grafiği, oyuncu tablosu, beceri
  kırılımı, ölümler). Ayarlarda "Geçmişi kaydet" ile kapatılabilir.

## 0.7.4 (2026-10-19)
### Yeni
- Her savaş için beceri kırılımı toplanıyor (vuruş, tick, kritik, kaçınma, ek vuruş, en büyük; perfect, block, back),
  saniyelik hasar ve iyileştirme serisi, ölüm ve diriliş anları (son vuranla). Şimdilik bellekte; dosyaya yazma ve rapor
  bağlantısı bir sonraki sürümde.

## 0.7.3 (2026-10-19)
### Yeni
- Biten savaşlar artık unutulmuyor: her savaşın özeti (süre, hedef, sonuç, toplam hasar ve iyileştirme, ölümler)
  oturum boyunca saklanır. Sunucu değişimi, kayıt sonu ve kapanış savaşı "kesildi" olarak kapatır. Kalıcı kayıt ve
  rapor bağlantısı sonraki sürümlerde.
### Düzeltmeler
- Savaş dışı iyileştirme artık tabloda satır açmıyor; iyileştirme savaş başlatmaz.

## 0.7.2 (2026-10-19)
### Düzeltmeler
- Zindana ya da yeni sunucuya geçince ilk 2 dakika kaybolmuyor: CanMeter yeni sunucuya birkaç saniye içinde geçer,
  geçiş sırasındaki mesajlar zamanıyla saklanır ve işlenir.

## 0.7.1 (2026-10-19)
### Düzeltmeler
- Rapor sol şeridinde filtre çipleri iki satıra ayrıldı (tür, sonuç); açık savaşın kartı belirgin.

## 0.7.0 (2026-10-17)
### Yeni
- Geçmiş raporu: `canmeter-rapor` aracı savaş kayıtlarından tek klasöre HTML rapor üretir; savaş listesi, DPS
  grafiği, oyuncu tablosu ve beceri kırılımı tarayıcıda açılır, sunucu gerekmez. Meter'ın kayıt yazması sonraki sürümde.

## 0.6.10 (2026-10-17)
### Düzeltmeler
- Aynı karakterin ikinci satır olarak geri gelmesi (görüş alanına yeniden girince) düzeltildi.

## 0.6.9 (2026-10-17)
### Düzeltmeler
- Aynı karakter iki satır olarak görünmüyor: bölge geçişinde yeni kimlik alan ya da sahibinin adını taşıyan varlıkların
  hasarı ve iyileştirmesi karakterin kendi satırına yazılır; ölüm ve diriliş de aynı satırda.

## 0.6.8 (2026-10-17)
### Düzeltmeler
- Kayıt aracı var olan dosyanın üstüne yazmaz; dosya adı saniye içerir.

## 0.6.7 (2026-10-17)
### Yeni
- CanMeter yeni bir derleme gördüğünde savaş bitince kendini yeniden başlatır; Ctrl+Shift+R (oyundayken de) hemen
  yeniden başlatır. Pencere aynı yerde ve ölçekte açılır.

## 0.6.6 (2026-10-17)
### Yeni
- Ayarlar'da "Hedef çubuğu" seçeneği: hedefin adı, HP çubuğu ve kalan süre satırı gizlenebilir. Varsayılan açık.

## 0.6.5 (2026-10-17)
### Düzeltmeler
- Kuşak görünümünde çubuk satırın sonuna kadar dolar; en yüksek hasarlı satırın çubuğu artık yüzde sütununda bitmez.
- Kuşak görünümünde çubuk tam doluyken iyileştirme yazısı DPS sayısının üstüne binmiyor; sayıların soluna dizilir.

## 0.6.4 (2026-10-17)
### Düzeltmeler
- CanMeter'ın kendi logosu var: exe simgesi, görev çubuğu ve Alt+Tab'da "Crimson Wings" amblemi.

## 0.6.3 (2026-10-17)
### Düzeltmeler
- PvP'de hedef seçilmiyor ve satır çıkmıyordu: son vurduğun rakip oyuncu artık hedef olur (ad, HP, kalan süre),
  kendi satırın ve parti satırları görünür.
- Parti üyesine yapılan iyileştirme artık hasar değil iyileştirme olarak sayılır.
- Hedefe yalnız kaçınılan vuruşla dokunup hasar vermeyen varlıklar (Sealed Dungeon'daki sıfırlı `#id` satırları) satır
  almaz.

## 0.6.2 (2026-10-16)
### Düzeltmeler
- Kuşak görünümünde çubuğun üstüne düşen ad, hasar, DPS ve iyileştirme yazıları artık koyu renkte; sarı ve açık yeşil
  sınıf çubuklarında okunmayan beyaz yazı sorunu giderildi. Yazı çubuğun ucunda bölünse bile iki yarısı da okunur.

## 0.6.1 (2026-10-16)
### Düzeltmeler
- Boyutlandırma tutamacı sağ alt köşede (üç çapraz çizgi); köşeden sürükleyince görünüm ölçeklenir.

## 0.6.0 (2026-10-16)
### Yenilikler
- Summon, pet, yer büyüsü ve skill efektlerinin hasarı ve iyileştirmesi artık sahibi olan oyuncunun satırına yazılır;
  boss savaşındaki `#id` satır seli bitti.
- Hedefin HP çubuğu spawn anından itibaren doğru; HP güncellemelerinin ikinci düzeni çözüldü.
### Düzeltmeler
- Ölüm sayacı yalnız oyuncular için artar (summon ölümleri sayılmaz).

## 0.5.3 (2026-10-16)
### Düzeltmeler
- Alan savaşında (raid) tanınmayan oyuncular listeyi dolduruyordu; artık parti dışından en çok 3 satır görünür.

## 0.5.2 (2026-10-16)
### Düzeltmeler
- Overlay'in üst kenarındaki ince açık çizgi kaldırıldı.

## 0.5.1 (2026-10-16)
### Düzeltmeler
- Ölü oyuncu rozeti "ÖLDÜ" yerine "ÖLÜ" (İngilizce "DEAD").

## 0.5.0 (2026-10-16)
### Yenilikler
- Ölüm sayacı: savaşta ölen oyuncunun satırında "†2" gibi kaç kez öldüğü görünür; "ÖLDÜ" rozeti yalnız o an ölüyken.
- Kilit düğmesi daha belirgin: asma kilit simgesi, kilitliyken renkli zemin.
### Düzeltmeler
- Yaratık spawn mesajı ölüm sanılıp moblar "ÖLDÜ" olarak işaretlenebiliyordu; artık yalnız gerçek ölüm mesajı sayılır.
- Bölgeden ayrılan (despawn) parti üyesi ölü sayılmaz; canı yeniden görülen oyuncu dirilmiş sayılır.

## 0.4.10 (2026-10-16)
### Düzeltmeler
- Demo verisinde savaşın ilk saniyesinde DPS değerleri hasardan büyük görünüyordu; düzeltildi.

## 0.4.9 (2026-10-16)
### Düzeltmeler
- Kuşak görünümünde yazıların altındaki gölge kaldırıldı; yazılar net.

## 0.4.8 (2026-10-16)
### Düzeltmeler
- Yeni görünüm "Kuşak": kalın, sınıf renginde satır şeritleri; solda daire içinde sınıf ikonu; sağda iyileştirme, toplam
  hasar, DPS ve pay; hedef satırında HP yüzdesi, kalan süre ve kırmızı HP çubuğu. Ayarlar > Görünüm'den ya da
  Ctrl+Shift+S ile seçilir. Diğer görünümler değişmedi.

## 0.4.7 (2026-10-16)
### Düzeltmeler
- Sınıf ikonları artık oyundaki gerçek ikonlar: bütün skin'lerde satırlarda sınıfın kendi ikonu görünür.

## 0.4.6 (2026-10-15)
### Düzeltmeler
- Bölge ya da kanal değiştirdikten sonra kendi satırınız kayboluyordu; artık hedefe vuran "tanınmayan" karakter (siz)
  listelenir. O durumda ad yerine `#kimlik` ve rozetsiz görünebilir; yeniden giriş tam adı getirir.
- Satırlar yalnız hasara göre sıralanır. Hedef artık sizin en son vurduğunuz düşman.

## 0.4.5 (2026-10-15)
### Düzeltmeler
- CanMeter oyuna girdikten sonra açıldıysa karakteriniz tanınmıyor ve 0.4.1'den beri satırlar boş kalıyordu. Artık o
  durumda hedefe vuranlar gösterilir (tek başınaysanız siz) ve overlay'de "bölge ya da kanal değiştir, ya da yeniden
  gir" ipucu çıkar; karakter tanınınca parti kuralı devreye girer.
- Oyun trafiği yokken overlay "Oyun trafiği bekleniyor" yazar.

## 0.4.4 (2026-10-15)
### Geliştirme
- Masaüstü deneme ve ölçüm betikleri `araclar/` altında; kullanıcıya görünen davranış değişmedi.

## 0.4.3 (2026-10-15)
### Düzeltmeler
- Overlay fareyle boyutlandırılır: sağ kenardaki ince tutamacı tutup sürükleyin; bütün görünüm orantılı
  büyür ya da küçülür, yükseklik içeriğe göre kendiliğinden ayarlanır. Kilitliyken tutamaç gizlidir.

## 0.4.2 (2026-10-15)
### Düzeltmeler
- Kilitliyken kilit düğmesi tıklanabilir kalır: fare düğmenin üstüne gelince overlay o noktada tıklamayı alır,
  gerisi oyuna geçmeye devam eder. Ctrl+Shift+K yine çalışır.

## 0.4.1 (2026-10-15)
### Düzeltmeler
- Overlay'de yalnız parti üyeleri görünür; tek başınaysanız yalnız siz. Size vuran yaratıklar artık listeye girmez.
- Parti üyelerinin sınıfları ve sınıf ikonları doğru gelir.

## 0.4.0 (2026-10-15)
### Yeni
- CanMeter artık oyundan canlı ölçüyor: program açıldığında ağ adaptörünü kendisi seçer, oyun sunucusunu
  bulur ve savaş başlayınca parti üyelerinin hasarını, DPS'ini, kritik oranını, iyileştirmesini ve hedefin
  kalan canını gösterir. Veri alınamazsa demo verisiyle devam eder (alt bilgide "Demo verisi").
- Geliştiriciler için: `--kayit <dosya.pcap>` ile kayıt oynatma, `--arayuz <ad>` ile adaptör seçimi, `--demo`.

## 0.3.7 (2026-10-15)
### Değişti
- Görünürde değişiklik yok: oyun paketlerini çözen iç katmanın ilk parçası (çerçeveleme ve akış birleştirme)
  eklendi, kayıt üzerinde doğrulandı. Canlı ölçüm bir sonraki adımda.

## 0.3.6 (2026-10-14)
### Değişti
- CanMeter açık kaynak: GPL-3.0 lisansı eklendi (`LICENSE`).

## 0.3.5 (2026-10-14)
### Düzeltmeler
- Overlay'i taşıdığınız yer hatırlanır; program bir sonraki açılışta aynı konumda açılır.

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
