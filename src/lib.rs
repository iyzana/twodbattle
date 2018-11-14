extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow;
pub use map::Map;
pub use map_controller::MapController;
pub use map_view::{MapView, MapViewSettings};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
pub use player::Player;
pub use player_controller::PlayerController;
pub use player_view::PlayerView;

mod map;
mod map_controller;
mod map_generator;
mod map_view;
mod player;
mod player_controller;
mod player_view;

pub fn run() {
    let opengl = OpenGL::V3_3;
    let mut window: GlutinWindow = WindowSettings::new("2dbattle", (1920, 1080))
        .exit_on_esc(true)
        .samples(16)
        .opengl(opengl)
        .fullscreen(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let map = Map::new();
    let mut map_controller = MapController::new(map);
    let map_view_settings = MapViewSettings::new();
    let map_view = MapView::new(map_view_settings);

    let player = Player::new(50.0, 50.0);
    let mut player_controller = PlayerController::new(player);
    let player_view = PlayerView::new();

    while let Some(event) = events.next(&mut window) {
        map_controller.event(&event);
        player_controller.event(&map_controller.map, &event);

        if let Some(r) = event.render_args() {
            gl.draw(r.viewport(), |c, g| {
                use graphics::clear;
                clear([0.0, 0.0, 0.0, 1.0], g);

                map_view.draw(&map_controller, &c, g);
                player_view.draw(&player_controller, &c, g);
            });
        }
    }
}
