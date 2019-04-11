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
