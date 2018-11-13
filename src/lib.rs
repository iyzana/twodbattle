extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::*;
use piston::window::WindowSettings;
use std::f32::consts;

pub struct App {
    gl: GlGraphics,
    rotation: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let (w, h) = (200.0, 200.0);

        self.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            clear([0.0; 4], gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-w / 2.0, -h / 2.0);

            let r = rotation as f32;
            let color = [
                r.sin(),
                (r + consts::PI * 2.0 / 3.0).sin(),
                (r + consts::PI * 4.0 / 3.0).sin(),
                1.0,
            ];
            let shape = rectangle::rectangle_by_corners(0.0, 0.0, w, h);
            rectangle(color, shape, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += consts::PI as f64 * args.dt;
    }
}

pub fn run() {
    let opengl = OpenGL::V3_3;
    let mut window: GlutinWindow = WindowSettings::new("2dbattle", [1920, 1080])
        .exit_on_esc(true)
        .opengl(opengl)
        .fullscreen(true)
        .decorated(false)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(event) = events.next(&mut window) {
        if let Some(r) = event.render_args() {
            app.render(&r);
        }

        if let Some(u) = event.update_args() {
            app.update(&u);
        }
    }
}
