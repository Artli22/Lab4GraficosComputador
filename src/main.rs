mod color;
mod controles;
mod framebuffer;
mod obj;
mod triangle;

use color::Relleno;
use controles::Controles;
use framebuffer::Framebuffer;
use obj::Model;
use raylib::prelude::*;
use triangle::{profundidad_en, triangle};

const SCREEN_WIDTH: i32 = 1200;
const SCREEN_HEIGHT: i32 = 800;
const COLOR_SELECCION: Color = Color::new(255, 0, 255, 255);

#[derive(Clone, Copy)]
struct View {
    yaw: f32,
    pitch: f32,
    distance: f32,
}

fn init() -> (RaylibHandle, RaylibThread) {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Software Renderer")
        .build();
    rl.set_target_fps(60);
    (rl, thread)
}

fn project(v: Vector3, view: &View, focal: f32) -> Option<Vector3> {
    let (sin_y, cos_y) = view.yaw.sin_cos();
    let (sin_p, cos_p) = view.pitch.sin_cos();

    let x = v.x * cos_y + v.z * sin_y;
    let z = -v.x * sin_y + v.z * cos_y;
    let y = v.y * cos_p - z * sin_p;
    let z = v.y * sin_p + z * cos_p;

    let depth = view.distance - z;
    if depth <= 0.1 {
        return None;
    }

    let scale = focal / depth;
    Some(Vector3::new(
        SCREEN_WIDTH as f32 / 2.0 + x * scale,
        SCREEN_HEIGHT as f32 / 2.0 - y * scale,
        depth,
    ))
}

fn render(framebuffer: &mut Framebuffer, model: &Model, view: &View, seleccionado: Option<usize>) {
    let focal = SCREEN_HEIGHT as f32;
    let resaltado = Relleno::Solido(COLOR_SELECCION);

    type Visible<'a> = ([Vector3; 3], [Vector2; 3], &'a Relleno);
    let mut opacos: Vec<Visible> = Vec::new();
    let mut translucidos: Vec<Visible> = Vec::new();

    for (indice, tri) in model.triangles.iter().enumerate() {
        let projected: Vec<Vector3> = tri
            .vertices
            .iter()
            .filter_map(|&v| project(v - model.center, view, focal))
            .collect();

        if projected.len() == 3 {
            let pantalla = [projected[0], projected[1], projected[2]];
            let relleno = if seleccionado == Some(indice) { &resaltado } else { &tri.relleno };
            if relleno.es_opaco() {
                opacos.push((pantalla, tri.uvs, relleno));
            } else {
                translucidos.push((pantalla, tri.uvs, relleno));
            }
        }
    }

    // Los translúcidos van al final y de atrás hacia adelante para mezclar bien
    let profundidad = |t: &Visible| t.0[0].z + t.0[1].z + t.0[2].z;
    translucidos.sort_by(|a, b| profundidad(b).total_cmp(&profundidad(a)));

    for (pantalla, uvs, relleno) in opacos.iter().chain(translucidos.iter()) {
        triangle(framebuffer, *pantalla, *uvs, relleno);
    }
}

// Índice del triángulo más cercano a la cámara bajo el punto de pantalla (x, y)
fn triangulo_en(model: &Model, view: &View, x: f32, y: f32) -> Option<usize> {
    let focal = SCREEN_HEIGHT as f32;
    let mut mejor: Option<(usize, f32)> = None;

    for (indice, tri) in model.triangles.iter().enumerate() {
        let projected: Vec<Vector3> = tri
            .vertices
            .iter()
            .filter_map(|&v| project(v - model.center, view, focal))
            .collect();
        if projected.len() != 3 {
            continue;
        }
        let pantalla = [projected[0], projected[1], projected[2]];
        if let Some(depth) = profundidad_en(pantalla, x, y) {
            if mejor.map_or(true, |(_, d)| depth < d) {
                mejor = Some((indice, depth));
            }
        }
    }
    mejor.map(|(indice, _)| indice)
}

fn main() {
    let (mut rl, thread) = init();

    let obj_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/Intento1Lab3.obj");
    let model = match Model::load_obj(obj_path) {
        Ok(model) if !model.triangles.is_empty() => model,
        Ok(_) => {
            eprintln!("El archivo no contiene triángulos: {}", obj_path);
            return;
        }
        Err(e) => {
            eprintln!("No se pudo leer {}: {}", obj_path, e);
            return;
        }
    };
    println!("Modelo cargado: {} triángulos", model.triangles.len());

    let mut framebuffer = Framebuffer::new(&mut rl, &thread, SCREEN_WIDTH, SCREEN_HEIGHT);
    framebuffer.set_background_color(Color::new(20, 20, 30, 255));
    let vista_inicial = View {
        yaw: 0.8,
        pitch: 0.4,
        distance: model.size.max(1.0) * 2.0,
    };
    let mut view = vista_inicial;
    let mut controles = Controles::new();
    let mut seleccionado: Option<usize> = None;

    while !rl.window_should_close() {
        let entrada = controles.leer(&rl);
        if entrada.reiniciar {
            view = vista_inicial;
        }
        if entrada.quitar_seleccion {
            seleccionado = None;
        }
        if let Some(clic) = entrada.clic {
            seleccionado = triangulo_en(&model, &view, clic.x, clic.y);
            if let Some(i) = seleccionado {
                println!("[{}]", i);
            }
        }
        view.yaw += entrada.yaw;
        view.pitch = (view.pitch + entrada.pitch).clamp(-1.5, 1.5);
        view.distance = (view.distance - entrada.zoom * model.size * 0.1).max(1.0);

        framebuffer.clear();
        render(&mut framebuffer, &model, &view, seleccionado);

        let mut d = rl.begin_drawing(&thread);
        framebuffer.render_buffer(&mut d);
        d.draw_fps(10, 10);
        d.draw_text("Flechas/WASD/Mouse/Stick: Rotar | Rueda/Q/E: Zoom | R: Reiniciar | ESC: Salir", 10, 30, 20, Color::WHITE);
        let texto = match seleccionado {
            Some(i) => format!("Triangulo [{}] (magenta) | Clic: seleccionar | C: quitar", i),
            None => "Clic sobre un triangulo para ver su indice".to_string(),
        };
        d.draw_text(&texto, 10, 55, 20, Color::YELLOW);
    }
}
