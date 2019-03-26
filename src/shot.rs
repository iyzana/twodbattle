use crate::Player;

pub struct Shot {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub owner: String,
}

impl Shot {
    pub fn new(x: f64, y: f64, dx: f64, dy: f64, owner: String) -> Shot {
        Shot { x, y, dx, dy, owner }
    }
}
