use graphics::{Context, Graphics};
use BattlefieldController;
use std::f32::consts;

pub struct BattlefieldViewSettings {}

impl BattlefieldViewSettings {
    pub fn new() -> BattlefieldViewSettings {
        BattlefieldViewSettings {}
    }
}

pub struct BattlefieldView {
    pub settings: BattlefieldViewSettings,
}

impl BattlefieldView {
    pub fn new(settings: BattlefieldViewSettings) -> BattlefieldView {
        BattlefieldView { settings }
    }

    pub fn draw<G: Graphics>(&self, controller: &BattlefieldController, c: &Context, g: &mut G) {
        use graphics::*;

        let rotation = controller.battlefield.rotation;
        let v = c.viewport.unwrap().window_size;
        let (x, y) = ((v[0] / 2) as f64, (v[1] / 2) as f64);
        let (w, h) = (200.0, 200.0);

        let transform = c
            .transform
            .trans(x, y)
            .rot_rad(rotation)
            .trans(-w / 2.0, -h / 2.0);

        let r = rotation as f32;
        let color = [
            r.sin(),
            (r + consts::PI * 2.0 / 3.0).sin(),
            (r + consts::PI * 4.0 / 3.0).sin(),
            1.0,
        ];
        let shape = rectangle::rectangle_by_corners(0.0, 0.0, w, h);
        rectangle(color, shape, transform, g);
    }
}
