//! Satıra sızan sınıf rengi: mockup `.row::before`, soldan %13 alfa ile başlar, genişliğin %52'sinde biter.
//! Veri çubuğu değildir; dört köşeli bir mesh ile gerçek gradyan çizilir (düz band bar gibi görünüyordu).

use egui::epaint::{Mesh, Vertex, WHITE_UV};
use egui::{Color32, Pos2, Rect, Shape};

pub(super) fn ciz(p: &egui::Painter, r: Rect, sinif: Color32, olu: bool) {
    let alfa = if olu { 12 } else { 33 };
    let sol = Color32::from_rgba_unmultiplied(sinif.r(), sinif.g(), sinif.b(), alfa);
    let sag = Color32::TRANSPARENT;
    let x_son = r.left() + r.width() * 0.52;
    let mut mesh = Mesh::default();
    for (pos, renk) in [
        (r.left_top(), sol),
        (Pos2::new(x_son, r.top()), sag),
        (Pos2::new(x_son, r.bottom()), sag),
        (r.left_bottom(), sol),
    ] {
        mesh.vertices.push(Vertex {
            pos,
            uv: WHITE_UV,
            color: renk,
        });
    }
    mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
    p.add(Shape::mesh(mesh));
}
