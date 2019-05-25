use piston::input::GenericEvent;
use std::net::SocketAddr;

pub struct NetworkHostController {}

impl NetworkHostController {
    pub fn new(port: u32) -> Self {
        Self {}
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if e.update_args().is_some() {}
    }
}
