use crate::map_generator;

#[derive(Default)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Vec<bool>>,
}

impl Map {
    pub fn new() -> Map {
        let width = 48;
        let height = 27;
        let cells = map_generator::generate_map(width, height);

        Map {
            width,
            height,
            cells,
        }
    }

    pub fn cell_at(&self, x: u32, y: u32) -> bool {
        self.cells[x as usize][y as usize]
    }

    pub fn pos_of(&self, x: u32, y: u32) -> [f64; 4] {
        let (cw, ch) = (
            1920.0 / f64::from(self.width),
            1080.0 / f64::from(self.height),
        );
        [f64::from(x) * cw, f64::from(y) * ch, cw, ch]
    }

    pub fn pos_from_screen_coords(&self, pos: [f64; 4]) -> (u32, u32) {
        (
            (pos[0] / 1920.0 * f64::from(self.width)) as u32,
            (pos[1] / 1080.0 * f64::from(self.height)) as u32,
        )
    }
}
