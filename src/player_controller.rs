use crate::collision;
use crate::collision::Collision;
use crate::player;
use crate::{Map, Player, ShotController};
use piston::input::GenericEvent;
use std::collections::HashMap;

pub struct PlayerController {
    pub players: HashMap<String, Player>,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        map: &Map,
        shot_controller: &mut ShotController,
        e: &E,
    ) {
        if let Some(tick) = e.update_args() {
            for player in self.players.values_mut() {
                if player.state.lives == 0 {
                    continue;
                }

                Self::update(player, tick.dt);
                Self::process_collision(player, map, tick.dt, shot_controller);
                Self::motion(player, tick.dt);
            }
        }
    }

    fn update(player: &mut Player, dt: f64) {
        let speed = 300.0;
        if player.inputs.left && !player.inputs.right {
            player.state.dx = player.state.dx.min(-speed);
        } else if !player.inputs.left && player.inputs.right {
            player.state.dx = player.state.dx.max(speed);
        } else {
            let friction = if player.on_ground { 16.0 } else { 4.0 };
            player.state.dx -= player.state.dx * friction * dt;
        }

        if player.inputs.jump && (player.on_ground || player.has_double_jump) {
            player.inputs.jump = false;

            if player.on_ground {
                player.state.dy = player.state.dy.min(-805.0);
            } else {
                player.has_double_jump = false;
                player.state.dy = player.state.dy.min(-405.0);
            }
        } else {
            player.state.dy += 1000.0 * dt;
        }
    }

    fn process_collision(
        player: &mut Player,
        map: &Map,
        dt: f64,
        shot_controller: &mut ShotController,
    ) {
        let cells: Vec<_> = map.all_cells().collect();
        match collision::check(player, &cells, dt) {
            Some(Collision::SIDE { x, y }) => {
                if x.is_some() {
                    player.state.dx = 0.0;
                }
                if let Some(cell) = y {
                    if player.state.dy > 0.0 {
                        player.state.y = cell.y - player.state.height;
                        player.on_ground = true;
                        player.has_double_jump = true;
                    }

                    player.state.dy = 0.0;
                } else {
                    player.on_ground = false;
                }
            }
            Some(Collision::CORNER { cell }) => {
                if player.state.dy > 0.0 {
                    player.on_ground = true;
                    player.has_double_jump = true;
                    player.state.y = cell.y - player.state.height;
                }
                player.state.dx = 0.0;
                player.state.dy = 0.0;
            }
            _ => {
                player.on_ground = false;
            }
        }

        for shot in &mut shot_controller.shots {
            if player.state.lives > 0 && collision::collides(player, shot) {
                shot.lives = 0;
                player.state.lives -= 1;
            }
        }
    }

    fn motion(player: &mut Player, dt: f64) {
        let player::State { x, y, dx, dy, .. } = &mut player.state;

        *x += *dx * dt;
        *y += *dy * dt;
    }
}
