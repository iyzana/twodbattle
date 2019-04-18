use crate::{Map, Player, ShotController};
use piston::input::{keyboard::Key, Button, ButtonArgs, ButtonState, GenericEvent};

pub struct PlayerController {
    pub player: Player,
    dx: f64,
    dy: f64,
    left: bool,
    right: bool,
    space: bool,
    jump: bool,
    on_ground: bool,
    has_double_jump: bool,
}

impl PlayerController {
    pub fn new(player: Player) -> PlayerController {
        PlayerController {
            player,
            dx: 0.0,
            dy: 0.0,
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
        let friction = if self.on_ground { 16.0 } else { 4.0 };

        self.dx -= self.dx * friction * dt;
        self.dy += 1000.0 * dt;

        let speed = 300.0;
        if self.left && !self.right {
            self.dx = self.dx.min(-speed);
        } else if !self.left && self.right {
            self.dx = self.dx.max(speed);
        }

        if self.jump && (self.on_ground || self.has_double_jump) {
            self.jump = false;

            if self.on_ground {
                self.dy = self.dy.min(-800.0);
            } else {
                self.has_double_jump = false;
                self.dy = self.dy.min(-400.0);
            }
        }
    }

    fn process_collision(&mut self, map: &Map, dt: f64, shot_controller: &mut ShotController) {
        let new_player_x = self.player.x + self.dx * dt;
        let new_player_y = self.player.y + self.dy * dt;
        let moved_x = [new_player_x, self.player.y, 20.0, 20.0];
        let moved_y = [self.player.x, new_player_y, 20.0, 20.0];

        let mut collides_x = false;
        let mut collides_y = false;
        self.on_ground = false;

        let cells = map.all_cells().collect::<Vec<_>>();
        if cells
            .iter()
            .any(|cell| Self::collides(moved_x, cell.bounds()))
        {
            self.dx = 0.0;
            collides_x = true;
        }
        if let Some(cell) = cells
            .iter()
            .find(|cell| Self::collides(moved_y, cell.bounds()))
        {
            if self.dy > 0.0 {
                self.player.y = cell.y - self.player.height;
                self.on_ground = true;
                self.has_double_jump = true;
            }

            self.dy = 0.0;
            collides_y = true;
        }

        if !collides_x && !collides_y {
            let moved_xy = [new_player_x, new_player_y, 20.0, 20.0];

            if cells
                .iter()
                .any(|cell| Self::collides(moved_xy, cell.bounds()))
            {
                self.on_ground = true;
                self.has_double_jump = true;
                self.dx = 0.0;
                self.dy = 0.0;
            }
        }

        for shot in &mut shot_controller.shots {
            if self.player.lives > 0 && Self::collides(self.player.bounds(), shot.bounds()) {
                shot.lives = 0;
                self.player.lives -= 1;
            }
        }
    }

    fn collides(a: [f64; 4], b: [f64; 4]) -> bool {
        a[0] < b[0] + b[2] && a[0] + a[2] > b[0] && a[1] < b[1] + b[3] && a[1] + a[3] > b[1]
    }

    fn motion(&mut self, dt: f64) {
        let Player { width, height, .. } = self.player;

        self.player.x += self.dx * dt;
        self.player.y += self.dy * dt;

        self.player.x = self.player.x.max(0.0).min(1920.0 - width);
        self.player.y = self.player.y.max(0.0).min(1080.0 - height);
    }

    fn on_input(&mut self, input: ButtonArgs) {
        if Button::Keyboard(Key::Space) == input.button {
            let pressed = input.state == ButtonState::Press;
            if pressed {
                if !self.space {
                    self.jump = true;
                }
            } else {
                self.jump = false;
            }
            self.space = pressed;
        }

        if Button::Keyboard(Key::A) == input.button {
            self.left = input.state == ButtonState::Press;
        }

        if Button::Keyboard(Key::D) == input.button {
            self.right = input.state == ButtonState::Press;
        }
    }
}
