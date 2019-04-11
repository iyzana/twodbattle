#[derive(Debug)]
pub struct Cell {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub state: bool,
}

impl Cell {
    pub fn bounds(&self) -> [f64; 4] {
        [self.x, self.y, self.w, self.h]
    }
}
