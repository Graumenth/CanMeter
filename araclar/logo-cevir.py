"""Uygulama logosunu exe ikonuna ve pencere ikonuna çevirir: logo-cevir.py <kaynak png> <ikonlar klasörü>.
Çıktı: canmeter.ico (16..256, BMP içerikli; rc.exe ile gömülür) ve logo-64.rgba (pencere ikonu, ham RGBA)."""
import os
import sys

from PIL import Image

kaynak, hedef = sys.argv[1], sys.argv[2]
im = Image.open(kaynak).convert("RGBA")
if im.size[0] != im.size[1]:
    raise SystemExit(f"{kaynak}: kare bekleniyor, bulunan {im.size}")
boyutlar = [(b, b) for b in (16, 24, 32, 48, 64, 128, 256)]
im.save(os.path.join(hedef, "canmeter.ico"), sizes=boyutlar, bitmap_format="bmp")
kucuk = im.resize((64, 64), Image.LANCZOS)
with open(os.path.join(hedef, "logo-64.rgba"), "wb") as f:
    f.write(kucuk.tobytes())
print("canmeter.ico", os.path.getsize(os.path.join(hedef, "canmeter.ico")), "logo-64.rgba", len(kucuk.tobytes()))
