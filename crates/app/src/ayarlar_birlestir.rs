//! Oturumdaki ayarları kalıcı kopyaya işleme. Komut satırıyla gelen geçici dil/skin seçimi,
//! kullanıcı o ayarı değiştirmediyse kayda sızmaz.

use crate::ayarlar::Ayarlar;
use crate::dil::Dil;

impl Ayarlar {
    /// Oturumdaki değişiklikleri `kalici`ye işler ve dosyaya yazar.
    pub fn kaliciye_yaz(&self, kalici: &mut Ayarlar, onceki_dil: Dil, skin_ad: &str) {
        if self.dil != onceki_dil {
            kalici.dil = self.dil;
        }
        if self.skin != skin_ad {
            kalici.skin.clone_from(&self.skin);
        }
        kalici.zemin_renk = self.zemin_renk;
        kalici.zemin_opaklik = self.zemin_opaklik;
        kalici.toplam_goster = self.toplam_goster;
        kalici.hedef_goster = self.hedef_goster;
        kalici.olcek = self.olcek;
        kalici.konum = self.konum;
        kalici.kaydet();
    }
}
