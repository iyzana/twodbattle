use crate::player;
use crate::PlayerController;
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
            let player::State {
                x,
                y,
                color,
                width,
                height,
                lives,
                ..
            } = player.state;

            if lives == 0 {
                continue;
            }

            let mut inner_color = color;
            inner_color[3] = f32::from(lives) / 20.0;
            let coords = [x, y, width, height];

            Rectangle::new_round(inner_color, 5.0).draw(coords, &c.draw_state, c.transform, g);
            Rectangle::new_round_border(color, 5.0, 1.0).draw(
                coords,
                &c.draw_state,
                c.transform,
                g,
            );
        }
    }
}
