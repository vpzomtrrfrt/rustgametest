extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::input::{RenderEvent, UpdateEvent};
use glutin_window::GlutinWindow as Window;
use graphics::Transformed;

struct Point2D {
    x: f64,
    y: f64,
}

struct Player {
    position: Point2D,
    rotation: f64,
    color: [f32; 4],
}

impl Player {
    fn render(&self, c: &graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        let area = c.viewport.unwrap().draw_size;
        let scale = std::cmp::min(area[0], area[1]) as f64 / 400.0;
        let transform = c.transform
            .trans(
                area[0] as f64 / 2.0 + self.position.x * scale,
                area[1] as f64 / 2.0 + self.position.y * scale,
            )
            .rot_rad(self.rotation);
        graphics::rectangle(
            self.color,
            graphics::rectangle::centered_square(0.0, 0.0, scale * 20.0),
            transform,
            gl,
        );
    }
}

struct App {
    players: std::vec::Vec<Player>,
}

impl App {
    fn render(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        const BGCOLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const COLOR1: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        println!("before");

        graphics::clear(BGCOLOR, gl);
        let transform = c.transform;
        graphics::rectangle(
            COLOR1,
            graphics::rectangle::square(0.0, 0.0, 50.0),
            transform,
            gl,
        );
        println!("inside");
        for player in &self.players {
            player.render(&c, gl);
        }
    }
}

fn main() {
    let gl_version = opengl_graphics::OpenGL::V3_2;

    let mut window: Window = piston::window::WindowSettings::new("rustgametest", [400, 400])
        .opengl(gl_version)
        .exit_on_esc(true)
        .srgb(false)
        .build()
        .unwrap();

    let mut gl = opengl_graphics::GlGraphics::new(gl_version);

    let mut app = App {
        players: vec![
            Player {
                position: Point2D { x: 0.0, y: 0.0 },
                rotation: 0.0,
                color: [1.0, 0.0, 0.0, 1.0],
            },
        ],
    };

    let mut events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, glo| app.render(c, glo));
        }

        if let Some(u) = e.update_args() {
            //app.update(&u);
            println!("update!");
        }
    }
}
