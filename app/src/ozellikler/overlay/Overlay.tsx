import { memo, useCallback, useState } from "react";
import { useSavas } from "../../kancalar/useSavas";
import { sure } from "../../ortak/sayi";
import { overlayKilit } from "../../servis/motor";
import { AltBilgi } from "./AltBilgi";
import { Baslik } from "./Baslik";
import { HedefCubugu } from "./HedefCubugu";
import { OyuncuSatiri } from "./OyuncuSatiri";

export const Overlay = memo(function Overlay() {
  const savas = useSavas();
  const [kilitli, kilitAyarla] = useState(false);
  const kilitDegistir = useCallback(() => {
    const yeniKilit = !kilitli;
    kilitAyarla(yeniKilit);
    void overlayKilit(yeniKilit);
  }, [kilitli]);

  if (!savas) return <main className="overlay"><Baslik kilitli={kilitli} sure="0:00" kilitDegistir={kilitDegistir} /><p className="bekleniyor">Savaş bekleniyor</p></main>;

  return (
    <main className={`overlay${savas.aktif ? "" : " pasif"}`}>
      <Baslik kilitli={kilitli} sure={sure(savas.sure_ms)} kilitDegistir={kilitDegistir} />
      {savas.hedef && <HedefCubugu hedef={savas.hedef} />}
      <div className="sutun-baslik"><span>#</span><span>Oyuncu</span><span>DPS</span><span>Pay</span></div>
      <section className="oyuncular">{savas.oyuncular.map((oyuncu, indeks) => <OyuncuSatiri key={oyuncu.id} oyuncu={oyuncu} sira={indeks + 1} />)}</section>
      <AltBilgi />
    </main>
  );
});
