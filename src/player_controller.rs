use crate::collision;
use crate::collision::Collision;
use crate::{Map, Player, ShotController};
use piston::input::{keyboard::Key, Button, ButtonArgs, ButtonState, GenericEvent};

pub struct PlayerController {
    pub player: Player,
    left: bool,
    right: bool,
    space: bool,
    jump: bool,
    on_ground: bool,
    has_double_jump: bool,
}

impl PlayerController {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            left: false,
            right: false,
            space: false,
            jump: false,
            on_ground: false,
            has_double_jump: true,
        }
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        map: &Map,
        shot_controller: &mut ShotController,
        e: &E,
    ) {
        if self.player.lives == 0 {
            return;
        }

        if let Some(tick) = e.update_args() {
            self.update(tick.dt);
            self.process_collision(map, tick.dt, shot_controller);
            self.motion(tick.dt);
        }

        if let Some(input) = e.button_args() {
            self.on_input(input);
        }
    }

    fn update(&mut self, dt: f64) {
        let speed = 300.0;
        if self.left && !self.right {
            self.player.dx = self.player.dx.min(-speed);
        } else if !self.left && self.right {
            self.player.dx = self.player.dx.max(speed);
        } else {
            let friction = if self.on_ground { 16.0 } else { 4.0 };
            self.player.dx -= self.player.dx * friction * dt;
        }

        if self.jump && (self.on_ground || self.has_double_jump) {
            self.jump = false;

            if self.on_ground {
                self.player.dy = self.player.dy.min(-805.0);
            } else {
                self.has_double_jump = false;
                self.player.dy = self.player.dy.min(-405.0);
            }
        } else {
            self.player.dy += 1000.0 * dt;
        }
    }

    fn process_collision(&mut self, map: &Map, dt: f64, shot_controller: &mut ShotController) {
        let cells: Vec<_> = map.all_cells().collect();
        match collision::check(&self.player, &cells, dt) {
            Some(Collision::SIDE { x, y }) => {
                if x.is_some() {
                    self.player.dx = 0.0;
                }
                if let Some(cell) = y {
                    if self.player.dy > 0.0 {
                        self.player.y = cell.y - self.player.height;
                        self.on_ground = true;
                        self.has_double_jump = true;
                    }

                    self.player.dy = 0.0;
                } else {
                    self.on_ground = false;
                }
            }
            Some(Collision::CORNER { cell }) => {
                if self.player.dy > 0.0 {
                    self.on_ground = true;
                    self.has_double_jump = true;
                    self.player.y = cell.y - self.player.height;
                }
                self.player.dx = 0.0;
                self.player.dy = 0.0;
            }
            _ => {
                self.on_ground = false;
            }
        }

        for shot in &mut shot_controller.shots {
            if self.player.lives > 0 && collision::collides(&self.player, shot) {
                shot.lives = 0;
                self.player.lives -= 1;
            }
        }
    }

    fn motion(&mut self, dt: f64) {
        let Player {
            ref mut x,
            ref mut y,
            dx,
            dy,
            width,
            height,
            ..
        } = self.player;

        *x += dx * dt;
        *y += dy * dt;

        *x = x.max(0.0).min(1920.0 - width);
        *y = y.max(0.0).min(1080.0 - height);
    }

    fn on_input(&mut self, input: ButtonArgs) {
        match input.button {
            Button::Keyboard(Key::Space) => {
                if input.state != ButtonState::Press {
                    self.jump = false;
                } else if !self.space {
                    self.jump = true;
                }

                self.space = input.state == ButtonState::Press;
            }
            Button::Keyboard(Key::A) => {
                self.left = input.state == ButtonState::Press;
            }
            Button::Keyboard(Key::D) => {
                self.right = input.state == ButtonState::Press;
            }
            _ => {}
        }
    }
}
