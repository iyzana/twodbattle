use crate::entity::Bounds;
use crate::ShotController;
use graphics::{Context, Graphics};

#[derive(Default)]
pub struct ShotView {}

impl ShotView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn draw<G: Graphics>(&self, controller: &ShotController, c: &Context, g: &mut G) {
        use graphics::*;

        for shot in controller.shots.values() {
            let ellipse = Ellipse::new(shot.color);
            ellipse.draw(shot.bounds(), &c.draw_state, c.transform, g);
        }
    }
}
