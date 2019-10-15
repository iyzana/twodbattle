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

use clap::{App, Arg, ArgGroup};
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

pub fn run() {
    let matches = App::new("twodbattle")
        .author("succcubbus")
        .arg(
            Arg::with_name("host")
                .long("host")
                .requires("port")
                .help("hosts a game"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .takes_value(true)
                .value_name("PORT")
                .default_value("62304")
                .help("port to host on"),
        )
        .arg(
            Arg::with_name("join")
                .long("join")
                .value_name("SERVER:PORT")
                .help("join an existing game")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("observe")
                .long("observe")
                .help("only watch the game, do not register a player"),
        )
        .group(
            ArgGroup::with_name("type")
                .args(&["host", "join"])
                .required(true),
        )
        .get_matches();

    let host = matches.is_present("host");
    let join_server = matches.value_of("join");
    let observe = matches.is_present("observe");

    let opengl = OpenGL::V3_3;
    let mut window: GlfwWindow = WindowSettings::new("2dbattle", (1920, 1080))
        .exit_on_esc(true)
        .samples(16)
        .fullscreen(false)
        .build()
        .unwrap();

    let mut event_settings = EventSettings::new();
    event_settings.ups_reset = 15;
    let mut events = Events::new(event_settings);
    let mut gl = GlGraphics::new(opengl);

    let map = Map::new();
    let mut map_controller = MapController::new(map);
    let map_view_settings = MapViewSettings::new();
    let map_view = MapView::new(map_view_settings);

    let mut player_controller = PlayerController::new();
    let player_view = PlayerView::new();

    let mut local_input_controller = if observe {
        None
    } else {
        let name = if join_server.is_some() { "client" } else { "host" };
        let player = Player::new(name.to_string(), 50.0, 50.0, [1.0, 0.0, 0.0, 1.0]);
        player_controller.players.insert(name.to_string(), player);
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
        ClientController::connect(addr.parse().unwrap(), "0.0.0.0:0".parse().unwrap()).unwrap()
    });

    while let Some(event) = events.next(&mut window) {
        if let Some(local_input_controller) = local_input_controller.as_mut() {
            let Size { width, height } = window.size();
            let scale = (width / 1920.0).min(height / 1080.0);
            let translate_x = (width - 1920.0 * scale) / 2.0;
            let translate_y = (height - 1080.0 * scale) / 2.0;

            local_input_controller.event(
                &event,
                &mut player_controller,
                (scale, translate_x, translate_y),
            );
        }
        if let Some(client) = client.as_mut() {
            client.event(
                &event,
                &mut player_controller,
                &mut map_controller,
                &mut shot_controller,
                &mut local_input_controller,
            );
        }
        if client.is_some() || host.is_some() {
            player_controller.event(&map_controller.map, &mut shot_controller, &event);
            shot_controller.event(&map_controller.map, &mut player_controller, &event);
        }
        if let Some(host) = host.as_mut() {
            map_controller.event(&event);
            host.event(
                &event,
                &mut player_controller,
                &shot_controller,
                &map_controller,
            );
        }

        if let Some(r) = event.render_args() {
            gl.draw(r.viewport(), |mut c, g| {
                use graphics::{clear, Transformed};
                clear([0.0, 0.0, 0.0, 1.0], g);

                let Size { width, height } = window.size();
                let scale = (width / 1920.0).min(height / 1080.0);
                let translate_x = (width - 1920.0 * scale) / 2.0;
                let translate_y = (height - 1080.0 * scale) / 2.0;
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
}
