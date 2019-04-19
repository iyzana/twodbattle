use crate::Map;
use piston::input::GenericEvent;

pub struct MapController {
    pub map: Map,
}

impl MapController {
    pub fn new(map: Map) -> Self {
        Self { map }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(_u) = e.update_args() {}
    }
}
