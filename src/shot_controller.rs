use crate::collision;
use crate::collision::Collision;
use crate::{Map, PlayerController, Shot};
use piston::input::{mouse::MouseButton, Button, ButtonArgs, ButtonState, GenericEvent};

#[derive(Default)]
pub struct ShotController {
    pub shots: Vec<Shot>,
    shoot: bool,
    click: bool,
    mouse_pos: [f64; 2],
}

impl ShotController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        map: &Map,
        player_controller: &PlayerController,
        e: &E,
    ) {
        fn process_collision(shot: &mut Shot, map: &Map, dt: f64) {
            let cells: Vec<_> = map.all_cells().collect();
            match collision::check(shot, &cells, dt) {
                Some(Collision::SIDE { x, y }) => {
                    if x.is_some() {
                        shot.dx = -shot.dx;
                    }
                    if y.is_some() {
                        shot.dy = -shot.dy;
                    }
                    shot.lives -= 1;
                }
                Some(Collision::CORNER { .. }) => {
                    shot.dx = -shot.dx;
                    shot.dy = -shot.dy;
                    shot.lives -= 1;
                }
                _ => {}
            }
        }

        fn collides(a: [f64; 4], b: [f64; 4]) -> bool {
            a[0] < b[0] + b[2] && a[0] + a[2] > b[0] && a[1] < b[1] + b[3] && a[1] + a[3] > b[1]
        }

        fn motion(shot: &mut Shot, dt: f64) {
            shot.x += shot.dx * dt;
            shot.y += shot.dy * dt;
        }

        if let Some(tick) = e.update_args() {
            self.update(player_controller);

            self.shots.retain(|shot| {
                shot.lives > 0 && collides(shot.bounds(), [0.0, 0.0, 1920.0, 1080.0])
            });

            for mut shot in &mut self.shots {
                process_collision(&mut shot, map, tick.dt);
                motion(&mut shot, tick.dt);
            }
        }

        if let Some(input) = e.button_args() {
            self.on_input(input);
        }

        if let Some(mouse_pos) = e.mouse_cursor_args() {
            self.mouse_pos = mouse_pos;
        }
    }

    fn update(&mut self, player_controller: &PlayerController) {
        if self.shoot && player_controller.player.lives > 0 {
            let player_x = player_controller.player.x;
            let player_y = player_controller.player.y;
            let dx = player_x - self.mouse_pos[0];
            let dy = player_y - self.mouse_pos[1];
            let angle = dy.atan2(dx);
            let speed = 800.0;
            let spawn_dist = 20.0;
            self.shots.push(Shot::new(
                player_x + spawn_dist * -angle.cos(),
                player_y + spawn_dist * -angle.sin(),
                speed * -angle.cos(),
                speed * -angle.sin(),
                player_controller.player.name.clone(),
                5,
            ));

            self.shoot = false;
        }
    }

    fn on_input(&mut self, input: ButtonArgs) {
        if Button::Mouse(MouseButton::Left) == input.button {
            let pressed = input.state == ButtonState::Press;
            if pressed {
                if !self.click {
                    self.shoot = true;
                }
            } else {
                self.shoot = false;
            }
            self.click = pressed;
        }
    }
}
