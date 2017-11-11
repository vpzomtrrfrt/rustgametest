extern crate opengl_graphics;
extern crate graphics;
extern crate piston;
extern crate glutin_window;

use piston::input::{RenderEvent, UpdateEvent};

use glutin_window::GlutinWindow as Window;

struct Player {
    position: f32,
    rotation: f32
}

impl Player {
    fn render(&mut self, c: graphics::Context) {}
}

struct App {
    gl: opengl_graphics::GlGraphics
}

impl App {
    fn render(&mut self, args: &piston::input::RenderArgs) {
        const BGCOLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const COLOR1: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(BGCOLOR, gl);
            let transform = c.transform;
            graphics::rectangle(COLOR1, graphics::rectangle::square(0.0, 0.0, 50.0), transform, gl);
        })
    }
}

fn main() {
    let gl_version = opengl_graphics::OpenGL::V2_1;

    let mut window: Window =
        piston::window::WindowSettings::new("rustgametest", [400, 400])
            .opengl(gl_version)
            .exit_on_esc(true)
            .srgb(false)
            .build()
            .unwrap();

    let mut app = App {
        gl: opengl_graphics::GlGraphics::new(gl_version)
    };

    let mut events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            //app.update(&u);
            println!("update!");
        }
    }
}
