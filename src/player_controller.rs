use crate::collision;
use crate::collision::Collision;
use crate::{Map, Player, ShotController};
use piston::input::GenericEvent;

pub struct PlayerController {
    pub players: Vec<Player>,
}

impl PlayerController {
    pub fn new(player: Player) -> Self {
        Self {
            players: vec![player],
        }
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        map: &Map,
        shot_controller: &mut ShotController,
        e: &E,
    ) {
        if let Some(tick) = e.update_args() {
            for player in &mut self.players {
                if player.lives == 0 {
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
            player.dx = player.dx.min(-speed);
        } else if !player.inputs.left && player.inputs.right {
            player.dx = player.dx.max(speed);
        } else {
            let friction = if player.on_ground { 16.0 } else { 4.0 };
            player.dx -= player.dx * friction * dt;
        }

        if player.inputs.jump && (player.on_ground || player.has_double_jump) {
            player.inputs.jump = false;

            if player.on_ground {
                player.dy = player.dy.min(-805.0);
            } else {
                player.has_double_jump = false;
                player.dy = player.dy.min(-405.0);
            }
        } else {
            player.dy += 1000.0 * dt;
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
                    player.dx = 0.0;
                }
                if let Some(cell) = y {
                    if player.dy > 0.0 {
                        player.y = cell.y - player.height;
                        player.on_ground = true;
                        player.has_double_jump = true;
                    }

                    player.dy = 0.0;
                } else {
                    player.on_ground = false;
                }
            }
            Some(Collision::CORNER { cell }) => {
                if player.dy > 0.0 {
                    player.on_ground = true;
                    player.has_double_jump = true;
                    player.y = cell.y - player.height;
                }
                player.dx = 0.0;
                player.dy = 0.0;
            }
            _ => {
                player.on_ground = false;
            }
        }

        for shot in &mut shot_controller.shots {
            if player.lives > 0 && collision::collides(player, shot) {
                shot.lives = 0;
                player.lives -= 1;
            }
        }
    }

    fn motion(player: &mut Player, dt: f64) {
        let Player {
            x,
            y,
            dx,
            dy,
            width,
            height,
            ..
        } = player;

        *x += *dx * dt;
        *y += *dy * dt;

        *x = x.max(0.0).min(1920.0 - *width);
        *y = y.max(0.0).min(1080.0 - *height);
    }
}
