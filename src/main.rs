mod controles;
mod framebuffer;
mod obj;
mod triangle;

use controles::Controles;
use framebuffer::Framebuffer;
use obj::Model;
use raylib::prelude::*;
use triangle::triangle;

const SCREEN_WIDTH: i32 = 1200;
const SCREEN_HEIGHT: i32 = 800;

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

fn set_color(framebuffer: &mut Framebuffer, color: Color) {
    framebuffer.set_current_color(color);
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

fn render(framebuffer: &mut Framebuffer, model: &Model, view: &View) {
    let focal = SCREEN_HEIGHT as f32;

    for tri in &model.triangles {
        let projected: Vec<Vector3> = tri
            .vertices
            .iter()
            .filter_map(|&v| project(v - model.center, view, focal))
            .collect();

        if projected.len() == 3 {
            set_color(framebuffer, tri.color);
            triangle(framebuffer, projected[0], projected[1], projected[2]);
        }
    }
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

    while !rl.window_should_close() {
        let entrada = controles.leer(&rl);
        if entrada.reiniciar {
            view = vista_inicial;
        }
        view.yaw += entrada.yaw;
        view.pitch = (view.pitch + entrada.pitch).clamp(-1.5, 1.5);
        view.distance = (view.distance - entrada.zoom * model.size * 0.1).max(1.0);

        framebuffer.clear();
        render(&mut framebuffer, &model, &view);

        let mut d = rl.begin_drawing(&thread);
        framebuffer.render_buffer(&mut d);
        d.draw_fps(10, 10);
        d.draw_text("Flechas/WASD/Mouse/Stick: Rotar | Rueda/Q/E: Zoom | R: Reiniciar | ESC: Salir", 10, 30, 20, Color::WHITE);
    }
}
