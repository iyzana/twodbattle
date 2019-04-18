use crate::entity::{Bounds, Speed};

pub struct Player {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub dx: f64,
    pub dy: f64,
    pub lives: u8,
}

impl Player {
    pub fn new(name: String, x: f64, y: f64) -> Player {
        Player {
            name,
            x,
            y,
            width: 20.0,
            height: 20.0,
            dx: 0.0,
            dy: 0.0,
            lives: 20,
        }
    }

    pub fn bounds(&self) -> [f64; 4] {
        [self.x, self.y, self.width, self.height]
    }
}

impl Bounds for Player {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn w(&self) -> f64 {
        self.width
    }
    fn h(&self) -> f64 {
        self.height
    }
}

impl Speed for Player {
    fn dx(&self) -> f64 {
        self.dx
    }
    fn dy(&self) -> f64 {
        self.dy
    }
}
