use crate::entity::Bounds;
use crate::MapController;
use graphics::{Context, Graphics};

#[derive(Default)]
pub struct MapViewSettings {}

impl MapViewSettings {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct MapView {
    pub settings: MapViewSettings,
}

impl MapView {
    pub fn new(settings: MapViewSettings) -> Self {
        Self { settings }
    }

    pub fn draw<G: Graphics>(&self, controller: &MapController, c: &Context, g: &mut G) {
        use graphics::*;

        let color = [1.0; 4];

        let rect = Rectangle::new(color);
        let border = Rectangle::new_border([0.0, 0.0, 0.0, 1.0], 2.0);

        controller.map.all_cells().for_each(|cell| {
            rect.draw(cell.bounds(), &c.draw_state, c.transform, g);
            border.draw(cell.bounds(), &c.draw_state, c.transform, g);
        });
    }
}
