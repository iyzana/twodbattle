use crate::Map;
use piston::input::{Button, ButtonArgs, ButtonState, GenericEvent, Key};

pub struct MapController {
    pub map: Map,
}

impl MapController {
    pub fn new(map: Map) -> Self {
        Self { map }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(ButtonArgs {
            button: Button::Keyboard(Key::R),
            state: ButtonState::Press,
            ..
        }) = e.button_args()
        {
            self.map = Map::new();
        }
    }
}
