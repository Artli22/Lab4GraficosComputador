// EJEMPLOS DE PERSONALIZACIÓN DE COLORES
// Copia cualquiera de estas funciones y reemplaza generate_color() en main.rs

// ============================================
// OPCIÓN 1: Colores Predefinidos por Índice
// ============================================
fn generate_color_predefined(index: u32) -> Color {
    match index % 8 {
        0 => Color::RED,
        1 => Color::GREEN,
        2 => Color::BLUE,
        3 => Color::YELLOW,
        4 => Color::MAGENTA,
        5 => Color::CYAN,
        6 => Color::ORANGE,
        _ => Color::PURPLE,
    }
}

// ============================================
// OPCIÓN 2: Gradiente de Grises
// ============================================
fn generate_color_grayscale(index: u32) -> Color {
    let gray = ((index as f32 % 256.0) as u8);
    Color::new(gray, gray, gray, 255)
}

// ============================================
// OPCIÓN 3: Arco Iris HSV (Mejorado)
// ============================================
fn generate_color_rainbow(index: u32) -> Color {
    let hue = (index as f32 * 2.0) % 360.0;  // Ciclo 2x más rápido
    let saturation = 1.0;  // Saturación máxima
    let value = 1.0;       // Brillo máximo
    
    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
        255,
    )
}

// ============================================
// OPCIÓN 4: Colores por Altura (Z)
// ============================================
// Necesitas pasar el vértice como parámetro
fn generate_color_by_height(vertex: &Vertex) -> Color {
    let height = ((vertex.z + 100.0) / 200.0).clamp(0.0, 1.0);
    Color::new(
        0,
        ((height * 255.0) as u8),
        (((1.0 - height) * 255.0) as u8),
        255,
    )
}

// ============================================
// OPCIÓN 5: Colores Pastel
// ============================================
fn generate_color_pastel(index: u32) -> Color {
    let hue = (index as f32 * 137.508) % 360.0;
    let saturation = 0.4;  // Baja saturación = colores pastel
    let value = 0.95;

    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
        255,
    )
}

// ============================================
// OPCIÓN 6: Colores Oscuros/Neón
// ============================================
fn generate_color_neon(index: u32) -> Color {
    let hue = (index as f32 * 45.0) % 360.0;  // Ciclo más lento
    let saturation = 1.0;
    let value = 0.7;  // Más oscuro

    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
        255,
    )
}

// ============================================
// OPCIÓN 7: Patrón Checkerboard
// ============================================
fn generate_color_checkerboard(index: u32) -> Color {
    if (index / 5) % 2 == 0 {
        Color::WHITE
    } else {
        Color::BLACK
    }
}

// ============================================
// OPCIÓN 8: Colores Aleatorios Seeded
// ============================================
fn generate_color_random(index: u32) -> Color {
    // Pseudo-random usando seed
    let seed = index.wrapping_mul(2654435761);
    let r = ((seed >> 8) & 0xFF) as u8;
    let g = ((seed >> 16) & 0xFF) as u8;
    let b = ((seed >> 24) & 0xFF) as u8;
    Color::new(r, g, b, 255)
}

// ============================================
// OPCIÓN 9: Gradiente de Calor (Heat Map)
// ============================================
fn generate_color_heatmap(index: u32) -> Color {
    let normalized = (index as f32 % 256.0) / 256.0;
    
    let (r, g, b) = if normalized < 0.25 {
        // Azul a Cian
        (0.0, normalized * 4.0, 1.0)
    } else if normalized < 0.5 {
        // Cian a Verde
        (0.0, 1.0, 1.0 - (normalized - 0.25) * 4.0)
    } else if normalized < 0.75 {
        // Verde a Amarillo
        ((normalized - 0.5) * 4.0, 1.0, 0.0)
    } else {
        // Amarillo a Rojo
        (1.0, 1.0 - (normalized - 0.75) * 4.0, 0.0)
    };

    Color::new(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        255,
    )
}

// ============================================
// OPCIÓN 10: Colores Corporativos
// ============================================
fn generate_color_corporate(index: u32) -> Color {
    let colors = vec![
        Color::new(0, 102, 204, 255),    // Azul corporativo
        Color::new(255, 153, 0, 255),    // Naranja
        Color::new(51, 153, 102, 255),   // Verde gris
        Color::new(204, 0, 0, 255),      // Rojo
        Color::new(102, 102, 153, 255),  // Gris azulado
    ];
    colors[(index as usize) % colors.len()]
}

// ============================================
// CÓMO USAR ESTAS FUNCIONES
// ============================================
/*
1. Copia una de estas funciones
2. En main.rs, reemplaza esta línea:
   
   let color = Self::generate_color(color_index);
   
   Por:
   
   let color = Self::generate_color_predefined(color_index);
   
   (o cualquier otra opción)

3. Recompila:
   cargo build --release

4. ¡Disfruta tus nuevos colores!

EJEMPLO COMPLETO:
En la línea ~70 de main.rs, dentro de "f" => {
Encuentra:
    let color = Self::generate_color(color_index);
    
Cambia a:
    let color = Self::generate_color_rainbow(color_index);
*/
