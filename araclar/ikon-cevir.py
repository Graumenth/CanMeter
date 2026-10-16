"""Sınıf ikonu PNG'lerini (40x42, RGBA) ham RGBA dosyasına çevirir: ikon-cevir.py <png klasörü> <çıktı klasörü>."""
import glob
import os
import sys

from PIL import Image

kaynak, hedef = sys.argv[1], sys.argv[2]
os.makedirs(hedef, exist_ok=True)
for yol in sorted(glob.glob(os.path.join(kaynak, "*.png"))):
    im = Image.open(yol).convert("RGBA")
    if im.size != (40, 42):
        raise SystemExit(f"{yol}: beklenen 40x42, bulunan {im.size}")
    ad = os.path.splitext(os.path.basename(yol))[0] + ".rgba"
    with open(os.path.join(hedef, ad), "wb") as f:
        f.write(im.tobytes())
    print(ad, len(im.tobytes()))
