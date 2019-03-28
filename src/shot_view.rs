use crate::{ShotController};
use graphics::{Context, Graphics};

#[derive(Default)]
pub struct ShotView {}

impl ShotView {
    pub fn new() -> ShotView {
        ShotView::default()
    }

    pub fn draw<G: Graphics>(&self, controller: &ShotController, c: &Context, g: &mut G) {
        use graphics::*;
        let color = [1.0, 0.0, 0.0, 1.0];
        let ellipse = Ellipse::new(color);

        for shot in &controller.shots {
            let coords = [shot.x, shot.y, 15.0, 15.0];

            ellipse.draw(coords, &c.draw_state, c.transform, g);
        }
    }
}
