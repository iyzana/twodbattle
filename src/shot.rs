use crate::entity::{Bounds, Speed};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Id {
    pub id: u32,
    pub owner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub id: Id,
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub lives: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shot {
    pub state: State,
    pub w: f64,
    pub h: f64,
    pub color: [f32; 4],
    pub dirty: bool,
}

impl Shot {
    pub fn new(x: f64, y: f64, dx: f64, dy: f64, id: u32, owner: String, color: [f32; 4]) -> Self {
        let id = Id { id, owner };
        Self::from_state(
            State {
                id,
                x,
                y,
                dx,
                dy,
                lives: 5,
            },
            color,
        )
    }

    pub fn from_state(state: State, color: [f32; 4]) -> Self {
        Self {
            state,
            w: 15.0,
            h: 15.0,
            color,
            dirty: true,
        }
    }
}

impl Bounds for Shot {
    fn x(&self) -> f64 {
        self.state.x
    }
    fn y(&self) -> f64 {
        self.state.y
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
        self.state.dx
    }
    fn dy(&self) -> f64 {
        self.state.dy
    }
}
