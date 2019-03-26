use piston::input::{mouse::MouseButton, Button, ButtonArgs, ButtonState, GenericEvent};
use crate::{Map, Shot};

pub struct ShotController {
    shots: Vec<Shot>,
    shoot: bool,
}

impl ShotController {
    pub fn new() -> ShotController {
        ShotController {
            shots: vec![],
            shoot: false,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, map: &Map, e: &E) {
        if let Some(tick) = e.update_args() {
            let shots: &mut Vec<Shot> = &mut self.shots;
            for shot in shots.iter_mut() {
                ShotController::update(shot, tick.dt);
                ShotController::check_collision(&shot, map, tick.dt);
                ShotController::motion(&shot, tick.dt);
            }
        }

        if let Some(input) = e.button_args() {
            self.on_input(input);
        }
    }

    fn update(shot: &mut Shot, dt: f64) {

    }

    fn check_collision(shot: &Shot, map: &Map, dt: f64) {}

    fn motion(shot: &Shot, dt: f64) {}

    fn on_input(&mut self, input: ButtonArgs) {
        if Button::Mouse(MouseButton::Left) == input.button {}
    }
}
