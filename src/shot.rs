use crate::entity::{Bounds, Speed};

pub struct Shot {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub dx: f64,
    pub dy: f64,
    pub owner: String,
    pub lives: u32,
}

impl Shot {
    pub fn new(x: f64, y: f64, dx: f64, dy: f64, owner: String, lives: u32) -> Shot {
        Shot {
            x,
            y,
            w: 15.0,
            h: 15.0,
            dx,
            dy,
            owner,
            lives,
        }
    }

    pub fn bounds(&self) -> [f64; 4] {
        [self.x, self.y, self.w, self.h]
    }
}

impl Bounds for Shot {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn w(&self) -> f64 {
        self.w
    }
    fn h(&self) -> f64 {
        self.h
    }
}

impl Speed for Shot {
    fn dx(&self) -> f64 {
        self.dx
    }
    fn dy(&self) -> f64 {
        self.dy
    }
}
