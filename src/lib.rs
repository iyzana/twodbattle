extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
use std::f32::consts;
pub use battlefield::Battlefield;
pub use battlefield_controller::BattlefieldController;
pub use battlefield_view::{BattlefieldView, BattlefieldViewSettings};

mod battlefield;
mod battlefield_controller;
mod battlefield_view;

pub fn run() {
    let opengl = OpenGL::V3_3;
    let mut window: GlutinWindow = WindowSettings::new("2dbattle", [1920, 1080])
        .exit_on_esc(true)
        .opengl(opengl)
        .fullscreen(true)
        .decorated(false)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let battlefield = Battlefield::new();
    let mut battlefield_controller = BattlefieldController::new(battlefield);
    let battlefield_view_settings = BattlefieldViewSettings::new();
    let battlefield_view = BattlefieldView::new(battlefield_view_settings);

    while let Some(event) = events.next(&mut window) {
        battlefield_controller.event(&event);

        if let Some(r) = event.render_args() {
            gl.draw(r.viewport(), |c, g| {
                use graphics::{clear};
                clear([0.0; 4], g);

                battlefield_view.draw(&battlefield_controller, &c, g);
            });
        }
    }
}
