use graphics::{Context, Graphics};
use Map;
use MapController;

pub struct MapViewSettings {}

impl MapViewSettings {
    pub fn new() -> MapViewSettings {
        MapViewSettings {}
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
        let [vw, vh] = c.get_view_size();
        let (w, h) = (vw / width as f64, vh / height as f64);
        let color = [1.0; 4];

        let rect = Rectangle::new(color);

        for x in 0..width {
            for y in 0..height {
                if controller.map.cell_at(x, y) {
                    let cell = [x as f64 * w, y as f64 * h, w as f64, h as f64];
                    rect.draw(cell, &c.draw_state, c.transform, g);
                }
            }
        }
    }
}
