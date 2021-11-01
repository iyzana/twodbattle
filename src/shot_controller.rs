use crate::cell::Cell;
use crate::collision;
use crate::collision::Collision;
use crate::entity::Bounds;
use crate::shot;
use crate::{Map, PlayerController, Shot};
use piston::input::GenericEvent;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Default)]
pub struct ShotController {
    pub shots: HashMap<shot::Id, Shot>,
    pub next_id: AtomicU32,
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
        fn process_collision(shot: &mut Shot, cells: &[Cell], dt: f64) {
            match collision::check(shot, cells, dt) {
                Some(Collision::Side { x, y }) => {
                    if x.is_some() {
                        shot.state.dx = -shot.state.dx;
                    }
                    if y.is_some() {
                        shot.state.dy = -shot.state.dy;
                    }
                    shot.state.lives -= 1;
                    shot.dirty = true;
                }
                Some(Collision::Corner { .. }) => {
                    shot.state.dx = -shot.state.dx;
                    shot.state.dy = -shot.state.dy;
                    shot.state.lives -= 1;
                    shot.dirty = true;
                }
                _ => {}
            }
        }

        fn collides(a: [f64; 4], b: [f64; 4]) -> bool {
            a[0] < b[0] + b[2] && a[0] + a[2] > b[0] && a[1] < b[1] + b[3] && a[1] + a[3] > b[1]
        }

        fn motion(shot: &mut Shot, dt: f64) {
            shot.state.x += shot.state.dx * dt;
            shot.state.y += shot.state.dy * dt;
        }

        if let Some(tick) = e.update_args() {
            self.update(map, player_controller);

            self.shots.retain(|_, shot| {
                shot.state.lives > 0 && collides(shot.bounds(), [0.0, 0.0, 1920.0, 1080.0])
            });

            let cells: Vec<_> = map.all_cells().collect();
            for mut shot in self.shots.values_mut() {
                process_collision(&mut shot, &cells, tick.dt);
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
                let id = self.next_id.fetch_add(1, Ordering::SeqCst);
                let shot = Shot::new(
                    player_x + spawn_dist * -angle.cos(),
                    player_y + spawn_dist * -angle.sin(),
                    speed * -angle.cos(),
                    speed * -angle.sin(),
                    id,
                    player.state.name.clone(),
                    player.state.color,
                );
                let cells: Vec<_> = map.all_cells().collect();
                if let Some(Collision::Side {
                    x: Some(_),
                    y: Some(_),
                }) = collision::check(&shot, &cells, 0.0)
                {
                    continue;
                }
                self.shots.insert(shot.state.id.clone(), shot);

                player.inputs.shoot = false;
            }
        }
    }
}
