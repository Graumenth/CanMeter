pub fn kacis(metin: &str) -> String {
    let mut sonuc = String::with_capacity(metin.len());
    for harf in metin.chars() {
        match harf {
            '&' => sonuc.push_str("&amp;"),
            '<' => sonuc.push_str("&lt;"),
            '>' => sonuc.push_str("&gt;"),
            '"' => sonuc.push_str("&quot;"),
            '\'' => sonuc.push_str("&#39;"),
            _ => sonuc.push(harf),
        }
    }
    sonuc
}
