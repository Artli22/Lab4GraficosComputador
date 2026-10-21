use crate::framebuffer::Framebuffer;
use raylib::prelude::Vector3;

fn edge(a: Vector3, b: Vector3, px: f32, py: f32) -> f32 {
    (b.x - a.x) * (py - a.y) - (b.y - a.y) * (px - a.x)
}

pub fn triangle(framebuffer: &mut Framebuffer, a: Vector3, b: Vector3, c: Vector3) {
    let area = edge(a, b, c.x, c.y);
    if area.abs() < 1e-6 {
        return;
    }

    let min_x = a.x.min(b.x).min(c.x).floor().max(0.0) as i32;
    let max_x = a.x.max(b.x).max(c.x).ceil().min((framebuffer.width - 1) as f32) as i32;
    let min_y = a.y.min(b.y).min(c.y).floor().max(0.0) as i32;
    let max_y = a.y.max(b.y).max(c.y).ceil().min((framebuffer.height - 1) as f32) as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;

            // Coordenadas baricéntricas; el píxel está dentro si las tres son >= 0
            let w0 = edge(b, c, px, py) / area;
            let w1 = edge(c, a, px, py) / area;
            let w2 = edge(a, b, px, py) / area;

            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let depth = w0 * a.z + w1 * b.z + w2 * c.z;
                framebuffer.point(x, y, depth);
            }
        }
    }
}
