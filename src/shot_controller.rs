use crate::{Map, PlayerController, Shot};
use piston::input::{mouse::MouseButton, Button, ButtonArgs, ButtonState, GenericEvent};

#[derive(Default)]
pub struct ShotController {
    pub shots: Vec<Shot>,
    shoot: bool,
    click: bool,
}

impl ShotController {
    pub fn new() -> ShotController {
        ShotController::default()
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        map: &Map,
        player_controller: &PlayerController,
        e: &E,
    ) {
        fn check_collision(shot: &mut Shot, map: &Map, dt: f64) {

            
        }

        fn collides(&self, a: [f64; 4], b: [f64; 4]) -> bool {
            a[0] < b[0] + b[2] && a[0] + a[2] > b[0] && a[1] < b[1] + b[3] && a[1] + a[3] > b[1]
        }

        fn motion(shot: &mut Shot, dt: f64) {
            shot.x += shot.dx * dt;
            shot.y += shot.dy * dt;
        }

        if let Some(tick) = e.update_args() {
            self.update(player_controller);

            let shots: &mut Vec<Shot> = &mut self.shots;
            for mut shot in shots.iter_mut() {
                check_collision(&mut shot, map, tick.dt);
                motion(&mut shot, tick.dt);
            }
        }

        if let Some(input) = e.button_args() {
            self.on_input(input);
        }
    }

    fn update(&mut self, player_controller: &PlayerController) {
        if self.shoot {
            self.shots.push(Shot {
                x: player_controller.player.x,
                y: player_controller.player.y,
                dx: 20.0,
                dy: 20.0,
                owner: player_controller.player.name.clone(),
            });

            self.shoot = false;
        }
    }

    fn on_input(&mut self, input: ButtonArgs) {
        if Button::Mouse(MouseButton::Left) == input.button {
            let pressed = input.state == ButtonState::Press;
            if pressed {
                if !self.click {
                    self.shoot = true;
                }
            } else {
                self.shoot = false;
            }
            self.click = pressed;
        }
    }
}
