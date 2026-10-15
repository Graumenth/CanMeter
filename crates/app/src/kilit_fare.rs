//! Kilitliyken pencere fare olayı almaz (geçirgen), bu yüzden egui imleci göremez. Ayrı iş parçacığı
//! imleci 40 ms'de bir okur; yalnız "kilit düğmesinin üstünde mi" durumu değişince kareyi uyandırır.
//! Kilit açıkken 250 ms uyur. Maliyet: GetCursorPos saniyede 25 kez, ölçülemeyecek kadar küçük.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const KILITLI_ARALIK: Duration = Duration::from_millis(40);
const ACIK_ARALIK: Duration = Duration::from_millis(250);

#[derive(Default)]
struct Durum {
    /// Kilit düğmesinin ekran dikdörtgeni, fiziksel piksel [sol, üst, sağ, alt]; `None`: kilit açık.
    alan: Mutex<Option<[i32; 4]>>,
    ustunde: AtomicBool,
}

pub struct FareGozcusu {
    durum: Arc<Durum>,
}

impl FareGozcusu {
    pub fn baslat(ctx: egui::Context) -> Self {
        let durum = Arc::new(Durum::default());
        let paylasilan = Arc::clone(&durum);
        let sonuc = std::thread::Builder::new()
            .name("canmeter-fare".into())
            .spawn(move || dongu(&paylasilan, &ctx));
        if let Err(hata) = sonuc {
            eprintln!("Fare gözcüsü başlatılamadı: {hata}");
        }
        Self { durum }
    }

    /// Kilitliyken her kare düğmenin ekran alanı verilir; kilit açılınca `None`.
    pub fn alan_ayarla(&self, alan: Option<[i32; 4]>) {
        if let Ok(mut a) = self.durum.alan.lock() {
            *a = alan;
        }
        if alan.is_none() {
            self.durum.ustunde.store(false, Ordering::Relaxed);
        }
    }

    /// Son okumada imleç kilit düğmesinin üstünde miydi.
    pub fn ustunde(&self) -> bool {
        self.durum.ustunde.load(Ordering::Relaxed)
    }
}

fn dongu(durum: &Durum, ctx: &egui::Context) {
    loop {
        let alan = durum.alan.lock().ok().and_then(|a| *a);
        let Some([sol, ust, sag, alt]) = alan else {
            std::thread::sleep(ACIK_ARALIK);
            continue;
        };
        let simdi =
            imlec().is_some_and(|(x, y)| (sol..sag).contains(&x) && (ust..alt).contains(&y));
        if durum.ustunde.swap(simdi, Ordering::Relaxed) != simdi {
            ctx.request_repaint();
        }
        std::thread::sleep(KILITLI_ARALIK);
    }
}

#[cfg(windows)]
fn imlec() -> Option<(i32, i32)> {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;
    let mut nokta = POINT { x: 0, y: 0 };
    // Yalnız çıkış parametresine yazar; işaretçi çağrı boyunca geçerli.
    (unsafe { GetCursorPos(&mut nokta) } != 0).then_some((nokta.x, nokta.y))
}

#[cfg(not(windows))]
fn imlec() -> Option<(i32, i32)> {
    None
}
