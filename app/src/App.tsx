import "./stil/tema.css";
import "./ozellikler/overlay/overlay.css";
import {
  Overlay,
} from "./ozellikler/overlay/Overlay";

/**
 * Uygulama yalnızca saydam overlay kökünü sunar.
 */
const uygulamaIcerigi = (
  <Overlay />
);

export default function App() {
  return uygulamaIcerigi;
}
