use piston::input::GenericEvent;
use crate::Map;

pub struct MapController {
    pub map: Map,
}

impl MapController {
    pub fn new(map: Map) -> MapController {
        MapController { map }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(u) = e.update_args() {}
    }
}
