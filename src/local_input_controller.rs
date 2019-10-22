use crate::player_controller::PlayerController;
use piston::input::{Button, ButtonState, GenericEvent, Key, MouseButton};

#[derive(Default)]
pub struct LocalInputController {
    pub local_player: String,
    space: bool,
}

impl LocalInputController {
    pub fn new(local_player: String) -> Self {
        Self {
            local_player,
            ..Self::default()
        }
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        e: &E,
        player_controller: &mut PlayerController,
        (scale, translate_x, translate_y): (f64, f64, f64),
    ) {
        let player = player_controller.players.get_mut(&self.local_player);

        let player = match player {
            Some(p) => p,
            None => return,
        };

        if let Some(input) = e.button_args() {
            match input.button {
                Button::Keyboard(Key::Space) => {
                    if input.state != ButtonState::Press {
                        player.dirty = true;
                        player.inputs.jump = false;
                    } else if !self.space {
                        player.dirty = true;
                        player.inputs.jump = true;
                    }

                    self.space = input.state == ButtonState::Press;
                }
                Button::Keyboard(Key::A) => {
                    let new_state = input.state == ButtonState::Press;
                    player.dirty = player.inputs.left != new_state;
                    player.inputs.left = new_state;
                }
                Button::Keyboard(Key::D) => {
                    let new_state = input.state == ButtonState::Press;
                    player.dirty = player.inputs.right != new_state;
                    player.inputs.right = new_state;
                }
                Button::Mouse(MouseButton::Left) => {
                    let new_state = input.state == ButtonState::Press;
                    player.dirty = player.inputs.shoot != new_state;
                    player.inputs.shoot = new_state;
                }
                _ => {}
            }
        }

        if let Some(mouse_pos) = e.mouse_cursor_args() {
            player.inputs.mouse_x = (mouse_pos[0] - translate_x) / scale;
            player.inputs.mouse_y = (mouse_pos[1] - translate_y) / scale;
            if player.inputs.shoot {
                player.dirty = true;
            }
        }
    }
}
