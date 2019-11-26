extern crate glfw_window;
extern crate graphics;
extern crate itertools;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod cell;
mod collision;
mod entity;
mod local_input_controller;
mod map;
mod map_controller;
mod map_generator;
mod map_view;
mod network;
mod player;
mod player_controller;
mod player_view;
mod shot;
mod shot_controller;
mod shot_view;

use clap::ArgMatches;
use glfw_window::GlfwWindow;
use local_input_controller::LocalInputController;
pub use map::Map;
pub use map_controller::MapController;
pub use map_view::{MapView, MapViewSettings};
use network::ClientController;
use network::HostController;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
use piston::window::{Size, Window};
pub use player::Player;
pub use player_controller::PlayerController;
pub use player_view::PlayerView;
pub use shot::Shot;
pub use shot_controller::ShotController;
pub use shot_view::ShotView;

pub fn run(matches: &ArgMatches) -> Result<(), anyhow::Error> {
    let host = matches.is_present("host");
    let join_server = matches.value_of("join");
    let observe = matches.is_present("observe");

    let mut window: GlfwWindow = WindowSettings::new("2dbattle", (1920, 1080))
        .exit_on_esc(true)
        .samples(16)
        .fullscreen(false)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(OpenGL::V3_3);

    let map = Map::new();
    let mut map_controller = MapController::new(map);
    let map_view_settings = MapViewSettings::new();
    let map_view = MapView::new(map_view_settings);

    let mut player_controller = PlayerController::new();
    let player_view = PlayerView::new();

    let mut local_input_controller = if observe {
        None
    } else {
        let name = matches.value_of("name").unwrap();
        if host {
            let color = player_controller
                .get_free_color()
                .expect("no colors available");
            let player = Player::new(name.to_string(), 50.0, 50.0, color);
            player_controller.players.insert(name.to_string(), player);
        }
        Some(LocalInputController::new(name.to_string()))
    };

    let mut shot_controller = ShotController::new();
    let shot_view = ShotView::new();

    let mut host = if host {
        let port = matches.value_of("port").expect("port is required");
        Some(HostController::listen(format!("0.0.0.0:{}", port)).unwrap())
    } else {
        None
    };
    let mut client = join_server.map(|addr| {
        let name = local_input_controller
            .as_ref()
            .map(|l| l.local_player.as_str());
        ClientController::connect(addr.parse().unwrap(), "0.0.0.0:0".parse().unwrap(), name)
            .unwrap()
    });

    fn get_scaling(window: &GlfwWindow) -> (f64, f64, f64) {
        let Size { width, height } = window.size();
        let scale = (width / 1920.0).min(height / 1080.0);
        let translate_x = (width - 1920.0 * scale) / 2.0;
        let translate_y = (height - 1080.0 * scale) / 2.0;
        (scale, translate_x, translate_y)
    }

    while let Some(event) = events.next(&mut window) {
        if let Some(local_input_controller) = local_input_controller.as_mut() {
            local_input_controller.event(&event, &mut player_controller, get_scaling(&window));
        }
        if let Some(client) = client.as_mut() {
            client.event(
                &event,
                &mut player_controller,
                &mut map_controller,
                &mut shot_controller,
                &mut local_input_controller,
            )?;
        }
        if let Some(host) = host.as_mut() {
            map_controller.event(&event);
            host.event(
                &event,
                &mut player_controller,
                &mut shot_controller,
                &mut map_controller,
            );
        }
        player_controller.event(&map_controller.map, &mut shot_controller, &event);
        shot_controller.event(&map_controller.map, &mut player_controller, &event);

        if let Some(r) = event.render_args() {
            gl.draw(r.viewport(), |mut c, g| {
                use graphics::{clear, Transformed};
                clear([0.0, 0.0, 0.0, 1.0], g);

                let (scale, translate_x, translate_y) = get_scaling(&window);
                // scale to window size and center
                c.transform = c
                    .transform
                    .trans(translate_x, translate_y)
                    .scale(scale, scale);

                map_view.draw(&map_controller, &c, g);
                player_view.draw(&player_controller, &c, g);
                shot_view.draw(&shot_controller, &c, g);
            });
        }
    }

    Ok(())
}
