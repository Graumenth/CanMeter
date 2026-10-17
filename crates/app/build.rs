//! Windows exe ikonu: `ikonlar/canmeter.ico` kaynak (RT_GROUP_ICON) olarak gömülür; Windows SDK'nın rc.exe'si
//! gerekir (MSVC araç zinciriyle gelir). Logo sahibin eseri, bkz. `ikonlar/KAYNAK.md`. Diğer platformlarda etkisiz.

fn main() {
    println!("cargo:rerun-if-changed=ikonlar/canmeter.ico");
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        let mut kaynak = winresource::WindowsResource::new();
        kaynak.set_icon("ikonlar/canmeter.ico");
        kaynak
            .compile()
            .expect("exe ikonu derlenemedi (rc.exe bulunamadı mı?)");
    }
}
