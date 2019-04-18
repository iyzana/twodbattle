use crate::entity::Bounds;

#[derive(Debug)]
pub struct Cell {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub state: bool,
}

impl Bounds for Cell {
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
