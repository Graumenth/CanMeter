// CanMeter raporu: satır açma, çip filtreleri, sütun sıralama. Bağımlılık yok.
document.addEventListener("click", function (e) {
  var satir = e.target.closest("tr.oyuncu");
  if (satir) {
    var k = satir.nextElementSibling;
    if (k && k.classList.contains("kirilim")) k.classList.toggle("acik");
    return;
  }
  var cip = e.target.closest(".cip");
  if (cip) {
    var grup = cip.dataset.grup, deger = cip.dataset.deger;
    cip.parentElement.querySelectorAll(".cip[data-grup='" + grup + "']").forEach(function (c) { c.classList.remove("secili"); });
    cip.classList.add("secili");
    var secimler = {};
    document.querySelectorAll(".cip.secili").forEach(function (c) { secimler[c.dataset.grup] = c.dataset.deger; });
    document.querySelectorAll(".kart").forEach(function (kart) {
      var goster = true;
      for (var g in secimler) if (secimler[g] !== "hepsi" && kart.dataset[g] !== secimler[g]) goster = false;
      kart.style.display = goster ? "" : "none";
    });
    return;
  }
  var th = e.target.closest("th[data-sira]");
  if (th) {
    var tablo = th.closest("table"), govde = tablo.tBodies[0], i = th.cellIndex, azalan = th.dataset.yon !== "azalan";
    var satirlar = Array.from(govde.querySelectorAll("tr.oyuncu, tr.beceri"));
    var cift = satirlar.map(function (s) { return [s, s.nextElementSibling && s.nextElementSibling.classList.contains("kirilim") ? s.nextElementSibling : null]; });
    cift.sort(function (a, b) {
      var x = parseFloat(a[0].cells[i].dataset.v || a[0].cells[i].textContent.replace(/[^\d.-]/g, "")) || 0;
      var y = parseFloat(b[0].cells[i].dataset.v || b[0].cells[i].textContent.replace(/[^\d.-]/g, "")) || 0;
      return azalan ? y - x : x - y;
    });
    cift.forEach(function (c) { govde.appendChild(c[0]); if (c[1]) govde.appendChild(c[1]); });
    tablo.querySelectorAll("th").forEach(function (h) { delete h.dataset.yon; });
    th.dataset.yon = azalan ? "azalan" : "artan";
  }
});
