extern crate glutin_window;
extern crate graphics;
extern crate itertools;
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
pub use shot::Shot;
pub use shot_controller::ShotController;
pub use shot_view::ShotView;

mod map;
mod map_controller;
mod map_generator;
mod map_view;
mod player;
mod player_controller;
mod player_view;
mod shot;
mod shot_controller;
mod shot_view;

pub fn run() {
    let opengl = OpenGL::V3_3;
    let mut window: GlutinWindow = WindowSettings::new("2dbattle", (1920, 1080))
        .exit_on_esc(true)
        .samples(0)
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

    let player = Player::new("succcubbus".to_string(), 50.0, 50.0);
    let mut player_controller = PlayerController::new(player);
    let player_view = PlayerView::new();

    let mut shot_controller = ShotController::new();
    let shot_view = ShotView::new();

    while let Some(event) = events.next(&mut window) {
        map_controller.event(&event);
        player_controller.event(&map_controller.map, &event);
        shot_controller.event(&map_controller.map, &player_controller, &event);

        if let Some(r) = event.render_args() {
            gl.draw(r.viewport(), |c, g| {
                use graphics::clear;
                clear([0.0, 0.0, 0.0, 1.0], g);

                map_view.draw(&map_controller, &c, g);
                player_view.draw(&player_controller, &c, g);
                shot_view.draw(&shot_controller, &c, g);
            });
        }
    }
}
