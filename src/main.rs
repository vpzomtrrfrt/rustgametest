extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

use piston::input::{RenderEvent, UpdateEvent};
use piston::input::controller::ControllerAxisEvent;
use sdl2_window::Sdl2Window as Window;
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
        let scale = std::cmp::min(area[0], area[1]) as f64;
        let transform = c.transform
            .trans(
                area[0] as f64 / 2.0 + self.position.x * scale,
                area[1] as f64 / 2.0 + self.position.y * scale,
            )
            .rot_rad(self.rotation);
        graphics::rectangle(
            self.color,
            graphics::rectangle::centered_square(0.0, 0.0, scale * 0.1),
            transform,
            gl,
        );
    }
    fn update(&mut self, time: f64, input: &InputState) {
        const SPEED: f64 = 0.4;
        const ROT_SPEED: f64 = 1.5;
        const AFTER_PAD: f64 = 1.05;
        if let Some(pos) = input.axes.get(&(0, 0)) {
            self.rotation += pos * time * ROT_SPEED;
        }
        self.position.x += self.rotation.cos() * time * SPEED;
        self.position.y += self.rotation.sin() * time * SPEED;
        while self.position.x > AFTER_PAD {
            self.position.x -= AFTER_PAD * 2.0;
        }
        while self.position.y > AFTER_PAD {
            self.position.y -= AFTER_PAD * 2.0;
        }
        while self.position.x < -AFTER_PAD {
            self.position.x += AFTER_PAD * 2.0;
        }
        while self.position.y < -AFTER_PAD {
            self.position.y += AFTER_PAD * 2.0;
        }
    }
}

struct App {
    players: std::vec::Vec<Player>,
    input: InputState,
}

impl App {
    fn render(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        const BGCOLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const COLOR1: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        graphics::clear(BGCOLOR, gl);
        let transform = c.transform;
        graphics::rectangle(
            COLOR1,
            graphics::rectangle::square(0.0, 0.0, 50.0),
            transform,
            gl,
        );
        for player in &self.players {
            player.render(&c, gl);
        }
    }
    fn update(&mut self, time: f64) {
        for mut player in &mut self.players {
            player.update(time, &self.input);
        }
    }
}

struct InputState {
    axes: std::collections::HashMap<(i32, u8), f64>,
}

impl InputState {
    fn new() -> InputState {
        return InputState {
            axes: std::collections::HashMap::new(),
        };
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
        input: InputState::new(),
    };

    let mut events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, glo| app.render(c, glo));
        }

        if let Some(u) = e.update_args() {
            app.update(u.dt);
        }

        if let Some(args) = e.controller_axis_args() {
            println!("input {} {} {}", args.axis, args.id, args.position);
            app.input.axes.insert((args.id, args.axis), args.position);
        }
    }
}
