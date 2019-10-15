use crate::collision;
use crate::collision::Collision;
use crate::{Map, PlayerController, Shot};
use piston::input::GenericEvent;

#[derive(Default)]
pub struct ShotController {
    pub shots: Vec<Shot>,
}

impl ShotController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        map: &Map,
        player_controller: &mut PlayerController,
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
            self.update(map, player_controller);

            self.shots.retain(|shot| {
                shot.lives > 0 && collides(shot.bounds(), [0.0, 0.0, 1920.0, 1080.0])
            });

            for mut shot in &mut self.shots {
                process_collision(&mut shot, map, tick.dt);
                motion(&mut shot, tick.dt);
            }
        }
    }

    fn update(&mut self, map: &Map, player_controller: &mut PlayerController) {
        for player in player_controller.players.values_mut() {
            if player.state.lives == 0 {
                continue;
            }

            if player.inputs.shoot {
                let player_x = player.state.x;
                let player_y = player.state.y;
                let mouse_x = player_x - player.inputs.mouse_x + 15.0 / 2.0;
                let mouse_y = player_y - player.inputs.mouse_y + 15.0 / 2.0;
                let angle = mouse_y.atan2(mouse_x);
                let speed = 800.0;
                let spawn_dist = 20.0;
                let shot = Shot::new(
                    player_x + spawn_dist * -angle.cos(),
                    player_y + spawn_dist * -angle.sin(),
                    speed * -angle.cos(),
                    speed * -angle.sin(),
                    player.state.name.clone(),
                    5,
                );
                let cells: Vec<_> = map.all_cells().collect();
                if let Some(Collision::SIDE {
                    x: Some(_),
                    y: Some(_),
                }) = collision::check(&shot, &cells, 0.0)
                {
                    continue;
                }
                self.shots.push(shot);

                player.inputs.shoot = false;
            }
        }
    }
}
