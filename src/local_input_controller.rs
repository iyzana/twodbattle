use crate::player_controller::PlayerController;
use piston::input::{Button, ButtonState, GenericEvent, Key, MouseButton};

#[derive(Default)]
pub struct LocalInputController {
    local_player: String,
    space: bool,
}

impl LocalInputController {
    pub fn new(local_player: String) -> Self {
        Self {
            local_player,
            ..Self::default()
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E, player_controller: &mut PlayerController) {
        let player = player_controller
            .players
            .get_mut(&self.local_player);

        let player = match player {
            Some(p) => p,
            None => return,
        };

        if let Some(input) = e.button_args() {
            match input.button {
                Button::Keyboard(Key::Space) => {
                    if input.state != ButtonState::Press {
                        player.inputs.jump = false;
                    } else if !self.space {
                        player.inputs.jump = true;
                    }

                    self.space = input.state == ButtonState::Press;
                }
                Button::Keyboard(Key::A) => {
                    player.inputs.left = input.state == ButtonState::Press;
                }
                Button::Keyboard(Key::D) => {
                    player.inputs.right = input.state == ButtonState::Press;
                }
                Button::Mouse(MouseButton::Left) => {
                    player.inputs.shoot = input.state == ButtonState::Press;
                }
                _ => {}
            }
        }

        if let Some(mouse_pos) = e.mouse_cursor_args() {
            let mouse_x = player.x - mouse_pos[0];
            let mouse_y = player.y - mouse_pos[1];
            player.inputs.angle = mouse_y.atan2(mouse_x);
        }
    }
}
