pub struct Player {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        Player {
            x,
            y,
            width: 20.0,
            height: 20.0,
        }
    }
}
