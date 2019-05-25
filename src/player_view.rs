use crate::{Player, PlayerController};
use graphics::{Context, Graphics};

#[derive(Default)]
pub struct PlayerView {}

impl PlayerView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn draw<G: Graphics>(&self, controller: &PlayerController, c: &Context, g: &mut G) {
        use graphics::*;

        for player in controller.players.values() {
            let Player {
                x,
                y,
                color,
                width,
                height,
                lives,
                ..
            } = *player;

            if lives == 0 {
                return;
            }

            let mut color = color;
            color[3] = f32::from(lives) / 20.0;
            let border_color = [1.0, 0.0, 0.0, 1.0];
            let coords = [x, y, width, height];

            Rectangle::new_round(color, 5.0).draw(coords, &c.draw_state, c.transform, g);
            Rectangle::new_round_border(border_color, 5.0, 1.0).draw(
                coords,
                &c.draw_state,
                c.transform,
                g,
            );
        }
    }
}
