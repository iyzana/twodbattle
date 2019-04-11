pub struct Shot {
    pub pos: [f64; 4],
    pub dx: f64,
    pub dy: f64,
    pub owner: String,
    pub lives: u32,
}

impl Shot {
    pub fn new(x: f64, y: f64, dx: f64, dy: f64, owner: String, lives: u32) -> Shot {
        Shot {
            pos: [x, y, 15.0, 15.0],
            dx,
            dy,
            owner,
            lives,
        }
    }
}
