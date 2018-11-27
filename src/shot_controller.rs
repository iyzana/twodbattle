use piston::input::{mouse::MouseButton, Button, ButtonArgs, ButtonState, GenericEvent};
use {Map, Shot};

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
            for shot in self.shots.iter_mut() {
                self.update(&shot, tick.dt);
                self.check_collision(&shot, map, tick.dt);
                self.motion(&shot, tick.dt);
            }
        }

        if let Some(input) = e.button_args() {
            self.on_input(input);
        }
    }

    fn update(&mut self, shot: &Shot, dt: f64) {}

    fn check_collision(&mut self, shot: &Shot, map: &Map, dt: f64) {}

    fn motion(&mut self, shot: &Shot, dt: f64) {}

    fn on_input(&mut self, input: ButtonArgs) {
        if Button::Mouse(MouseButton::Left) == input.button {}
    }
}
