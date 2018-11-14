use map_generator;

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Vec<bool>>,
}

impl Map {
    pub fn new() -> Map {
        let width: u32 = 48;
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
}
