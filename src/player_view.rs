use graphics::{Context, Graphics};
use crate::{Player, PlayerController};

#[derive(Default)]
pub struct PlayerView {}

impl PlayerView {
    pub fn new() -> PlayerView {
        PlayerView::default()
    }

    pub fn draw<G: Graphics>(&self, controller: &PlayerController, c: &Context, g: &mut G) {
        use graphics::*;

        let Player {
            x,
            y,
            width,
            height,
            ..
        } = controller.player;

        let color = [1.0, 0.0, 0.0, 1.0];
        let coords = [x, y, width, height];

        Rectangle::new_round(color, 5.0).draw(coords, &c.draw_state, c.transform, g);
    }
}
