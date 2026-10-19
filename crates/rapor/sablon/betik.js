// CanMeter raporu: satır açma, çip filtreleri, sütun sıralama. Bağımlılık yok.
function filtrele() {
  var secimler = {};
  document.querySelectorAll(".cip.secili").forEach(function (c) { secimler[c.dataset.grup] = c.dataset.deger; });
  var bolge = document.querySelector(".bolge-sec");
  if (bolge) secimler.bolge = bolge.value;
  document.querySelectorAll(".kart").forEach(function (kart) {
    var goster = true;
    for (var g in secimler) if (secimler[g] !== "hepsi" && kart.dataset[g] !== secimler[g]) goster = false;
    kart.style.display = goster ? "" : "none";
  });
  document.querySelectorAll(".kosu").forEach(function (baslik) {
    var kart = baslik.nextElementSibling, goster = false;
    while (kart && !kart.classList.contains("kosu")) {
      if (kart.classList.contains("kart") && kart.style.display !== "none") goster = true;
      kart = kart.nextElementSibling;
    }
    baslik.style.display = goster ? "" : "none";
  });
}
document.addEventListener("change", function (e) {
  if (e.target.matches(".bolge-sec")) filtrele();
});
document.addEventListener("click", function (e) {
  var satir = e.target.closest("tr.oyuncu");
  if (satir) {
    var k = satir.nextElementSibling;
    if (k && k.classList.contains("kirilim")) k.classList.toggle("acik");
    return;
  }
  var cip = e.target.closest(".cip");
  if (cip) {
    var grup = cip.dataset.grup;
    cip.parentElement.querySelectorAll(".cip[data-grup='" + grup + "']").forEach(function (c) { c.classList.remove("secili"); });
    cip.classList.add("secili");
    filtrele();
    return;
  }
  var th = e.target.closest("th[data-sira]");
  if (th) {
    var tablo = th.closest("table"), govde = tablo.tBodies[0], i = th.cellIndex, azalan = th.dataset.yon !== "azalan";
    var satirlar = Array.from(govde.rows).filter(function (s) { return !s.classList.contains("kirilim"); });
    var cift = satirlar.map(function (s) { return [s, s.nextElementSibling && s.nextElementSibling.classList.contains("kirilim") ? s.nextElementSibling : null]; });
    cift.sort(function (a, b) {
      var x = a[0].cells[i], y = b[0].cells[i];
      var fark = x.hasAttribute("data-v") && y.hasAttribute("data-v")
        ? Number(x.dataset.v) - Number(y.dataset.v)
        : x.textContent.localeCompare(y.textContent, "tr", { numeric: true });
      return azalan ? -fark : fark;
    });
    cift.forEach(function (c) { govde.appendChild(c[0]); if (c[1]) govde.appendChild(c[1]); });
    tablo.querySelectorAll("th").forEach(function (h) { delete h.dataset.yon; });
    th.dataset.yon = azalan ? "azalan" : "artan";
  }
});
