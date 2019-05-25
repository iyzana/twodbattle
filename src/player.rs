use crate::entity::{Bounds, Speed};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Inputs {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub shoot: bool,
    pub mouse_x: f64,
    pub mouse_y: f64,
}

pub struct Player {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: [f32; 4],
    pub dx: f64,
    pub dy: f64,
    pub lives: u8,

    pub inputs: Inputs,
    pub on_ground: bool,
    pub has_double_jump: bool,
}

impl Player {
    pub fn new(name: String, x: f64, y: f64, color: [f32; 4]) -> Self {
        Self {
            name,
            x,
            y,
            width: 20.0,
            height: 20.0,
            color,
            dx: 0.0,
            dy: 0.0,
            lives: 20,
            inputs: Inputs::default(),
            on_ground: false,
            has_double_jump: true,
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
