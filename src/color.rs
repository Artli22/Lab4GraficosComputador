use raylib::prelude::{Color, Vector2};

// Opacidad con la que se generan los colores provisionales (0 = invisible, 255 = opaco)
pub const ALPHA_POR_DEFECTO: u8 = 255;

// Cómo se obtiene el color de cada píxel de un triángulo.
// Para agregar texturas: añadir una variante (p. ej. Textura(Image)) y resolverla en `color_en`.
#[derive(Clone)]
pub enum Relleno {
    Solido(Color),
}

impl Relleno {
    // `uv` son las coordenadas de textura interpoladas en el píxel; el relleno sólido las ignora
    pub fn color_en(&self, _uv: Vector2) -> Color {
        match self {
            Relleno::Solido(color) => *color,
        }
    }

    // Los rellenos opacos escriben en el z-buffer; los translúcidos se mezclan con lo ya dibujado
    pub fn es_opaco(&self) -> bool {
        match self {
            Relleno::Solido(color) => color.a == 255,
        }
    }
}

// Colores manuales: (índice de triángulo, color). Sobrescriben el color generado.
// El índice se ve en pantalla y en la terminal al hacer clic sobre el triángulo.
// Ejemplo: (12, Color::new(255, 0, 0, 255)) pinta el triángulo 12 de rojo.
pub const COLORES_MANUALES: &[(usize, Color)] = &[
    (12, Color::new(255, 255, 255, 255)),
    (41, Color::new(40, 40, 50, 130)),
];

// Punto único para decidir el relleno de cada triángulo del modelo
pub fn asignar_relleno(indice: usize) -> Relleno {
    let manual = COLORES_MANUALES.iter().find(|(i, _)| *i == indice);
    match manual {
        Some((_, color)) => Relleno::Solido(*color),
        None => Relleno::Solido(generate_color(indice as u32, ALPHA_POR_DEFECTO)),
    }
}

// Mezcla `origen` sobre un píxel RGBA existente: resultado = origen * a + destino * (1 - a)
pub fn mezclar(origen: Color, destino: [u8; 4]) -> [u8; 4] {
    let a = origen.a as f32 / 255.0;
    let canal = |s: u8, d: u8| (s as f32 * a + d as f32 * (1.0 - a)).round() as u8;
    [
        canal(origen.r, destino[0]),
        canal(origen.g, destino[1]),
        canal(origen.b, destino[2]),
        255,
    ]
}

pub fn generate_color(index: u32, alpha: u8) -> Color {
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
        alpha,
    )
}
