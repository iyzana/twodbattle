use piston::input::GenericEvent;
use Battlefield;
use std::f32::consts;

pub struct BattlefieldController {
    pub battlefield: Battlefield,
}

impl BattlefieldController {
    pub fn new(battlefield: Battlefield) -> BattlefieldController {
        BattlefieldController { battlefield }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(u) = e.update_args() {
            self.battlefield.rotation += consts::PI as f64 * u.dt;
        }
    }
}
