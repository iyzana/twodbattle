use crate::{Map, PlayerController, Shot};
use itertools::Itertools;
use piston::input::{mouse::MouseButton, Button, ButtonArgs, ButtonState, GenericEvent};

#[derive(Default)]
pub struct ShotController {
    pub shots: Vec<Shot>,
    shoot: bool,
    click: bool,
    mouse_pos: [f64; 2],
}

impl ShotController {
    pub fn new() -> ShotController {
        ShotController::default()
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        map: &Map,
        player_controller: &PlayerController,
        e: &E,
    ) {
        fn check_collision(shot: &mut Shot, map: &Map, dt: f64) {
            let (cell_x, cell_y) = map.pos_from_screen_coords(shot.pos);
            let cells: Vec<[f64; 4]> = (cell_x.max(1) - 1..(cell_x + 2).min(map.width))
                .cartesian_product(cell_y.max(1) - 1..(cell_y + 2).min(map.height))
                .filter(|(x, y)| map.cell_at(*x, *y))
                .map(|(x, y)| map.pos_of(x, y))
                .collect();

            let new_shot_x = shot.pos[0] + shot.dx * dt;
            let new_shot_y = shot.pos[1] + shot.dy * dt;
            let moved_x = [new_shot_x, shot.pos[1], 15.0, 15.0];
            let moved_y = [shot.pos[0], new_shot_y, 15.0, 15.0];

            let mut collides_x = false;
            let mut collides_y = false;

            if cells.iter().any(|c| collides(moved_x, *c)) {
                shot.dx = -shot.dx;
                collides_x = true;
            }

            if cells.iter().any(|c| collides(moved_y, *c)) {
                shot.dy = -shot.dy;
                collides_y = true;
            }

            if !collides_x && !collides_y {
                let moved_xy = [new_shot_x, new_shot_y, 15.0, 15.0];

                if cells.iter().any(|c| collides(moved_xy, *c)) {
                    shot.dx = -shot.dx;
                    shot.dy = -shot.dy;
                    collides_x = true;
                    collides_y = true;
                }
            }

            if collides_x || collides_y {
                shot.lives -= 1;
            }
        }

        fn collides(a: [f64; 4], b: [f64; 4]) -> bool {
            a[0] < b[0] + b[2] && a[0] + a[2] > b[0] && a[1] < b[1] + b[3] && a[1] + a[3] > b[1]
        }

        fn motion(shot: &mut Shot, dt: f64) {
            shot.pos[0] += shot.dx * dt;
            shot.pos[1] += shot.dy * dt;
        }

        if let Some(tick) = e.update_args() {
            self.update(player_controller);

            for mut shot in self.shots.iter_mut() {
                check_collision(&mut shot, map, tick.dt);
                motion(&mut shot, tick.dt);
            }

            self.shots
                .retain(|shot| shot.lives > 0 && collides(shot.pos, [0.0, 0.0, 1920.0, 1080.0]));
        }

        if let Some(input) = e.button_args() {
            self.on_input(input);
        }

        if let Some(mouse_pos) = e.mouse_cursor_args() {
            self.mouse_pos = mouse_pos;
        }
    }

    fn update(&mut self, player_controller: &PlayerController) {
        if self.shoot {
            let player_x = player_controller.player.x;
            let player_y = player_controller.player.y;
            let dx = player_x - self.mouse_pos[0];
            let dy = player_y - self.mouse_pos[1];
            let angle = dy.atan2(dx);
            let speed = 800.0;
            self.shots.push(Shot::new(
                player_x,
                player_y,
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
