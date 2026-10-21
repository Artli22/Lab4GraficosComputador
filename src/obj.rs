use raylib::prelude::{Color, Vector3};
use std::fs;

pub struct Triangle {
    pub vertices: [Vector3; 3],
    pub color: Color,
}

pub struct Model {
    pub triangles: Vec<Triangle>,
    pub center: Vector3,
    pub size: f32,
}

impl Model {
    pub fn load_obj(path: &str) -> std::io::Result<Model> {
        let content = fs::read_to_string(path)?;
        let mut vertices: Vec<Vector3> = Vec::new();
        let mut triangles: Vec<Triangle> = Vec::new();

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts.first().copied() {
                Some("v") if parts.len() >= 4 => {
                    let coord = |i: usize| parts[i].parse::<f32>().unwrap_or(0.0);
                    vertices.push(Vector3::new(coord(1), coord(2), coord(3)));
                }
                Some("f") => {
                    // Formatos v, v/vt, v//vn y v/vt/vn: solo importa el índice de vértice
                    let indices: Vec<usize> = parts[1..]
                        .iter()
                        .filter_map(|p| p.split('/').next()?.parse::<usize>().ok())
                        .filter(|&i| i > 0 && i <= vertices.len())
                        .map(|i| i - 1)
                        .collect();

                    // Las caras con más de 3 vértices se dividen en abanico
                    for i in 1..indices.len().saturating_sub(1) {
                        triangles.push(Triangle {
                            vertices: [vertices[indices[0]], vertices[indices[i]], vertices[indices[i + 1]]],
                            color: generate_color(triangles.len() as u32),
                        });
                    }
                }
                _ => {}
            }
        }

        let (center, size) = bounds(&triangles);
        Ok(Model { triangles, center, size })
    }
}

fn bounds(triangles: &[Triangle]) -> (Vector3, f32) {
    let mut min = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = Vector3::new(f32::MIN, f32::MIN, f32::MIN);

    for v in triangles.iter().flat_map(|t| t.vertices.iter()) {
        min = Vector3::new(min.x.min(v.x), min.y.min(v.y), min.z.min(v.z));
        max = Vector3::new(max.x.max(v.x), max.y.max(v.y), max.z.max(v.z));
    }

    let center = (min + max) * 0.5;
    let size = (max.x - min.x).max(max.y - min.y).max(max.z - min.z);
    (center, size)
}

fn generate_color(index: u32) -> Color {
    let hue = (index as f32 * 137.508) % 360.0;
    let saturation = 0.7 + ((index as f32 * 0.1) % 0.3);
    let value = 0.8 + ((index as f32 * 0.05) % 0.2);

    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = match (hue / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Color::new(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
        255,
    )
}
