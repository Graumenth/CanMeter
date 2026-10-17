fn main() {
    if let Err(hata) = canmeter_rapor::komut::calistir() {
        eprintln!("Hata: {hata}");
        std::process::exit(1);
    }
}
