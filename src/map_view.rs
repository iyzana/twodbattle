use graphics::{Context, Graphics};
use crate::{Map, MapController};

#[derive(Default)]
pub struct MapViewSettings {}

impl MapViewSettings {
    pub fn new() -> MapViewSettings {
        MapViewSettings::default()
    }
}

pub struct MapView {
    pub settings: MapViewSettings,
}

impl MapView {
    pub fn new(settings: MapViewSettings) -> MapView {
        MapView { settings }
    }

    pub fn draw<G: Graphics>(&self, controller: &MapController, c: &Context, g: &mut G) {
        use graphics::*;

        let Map { width, height, .. } = controller.map;
        let (w, h) = (1920.0 / f64::from(width), 1080.0 / f64::from(height));
        let color = [1.0; 4];

        let rect = Rectangle::new(color);
        let border = Rectangle::new_border([0.0, 0.0, 0.0, 1.0], 5.0);

        for x in 0..width {
            for y in 0..height {
                if controller.map.cell_at(x, y) {
                    let cell = [f64::from(x) * w, f64::from(y) * h, w as f64, h as f64];
                    rect.draw(cell, &c.draw_state, c.transform, g);
                    border.draw(cell, &c.draw_state, c.transform, g);
                }
            }
        }
    }
}
