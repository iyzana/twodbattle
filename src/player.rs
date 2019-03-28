pub struct Player {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Player {
    pub fn new(name: String, x: f64, y: f64) -> Player {
        Player {
            name,
            x,
            y,
            width: 20.0,
            height: 20.0,
        }
    }
}
