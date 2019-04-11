use rand::Rng;
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub fn generate_map(width: u32, height: u32) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    let num_walls = 16;
    let mut walls: Vec<Wall> = vec![];
    let min_width = width as f32 * 0.3;
    let max_width = width as f32 * 0.5;
    let min_height = height as f32 * 0.3;
    let max_height = height as f32 * 0.5;

    walls.push(Wall(0, 0, width, true));
    walls.push(Wall(0, height - 1, width, true));
    walls.push(Wall(0, 0, height, false));
    walls.push(Wall(width - 1, 0, height, false));

    while walls.len() < num_walls {
        let x = rng.gen_range(1, width - 1);
        let y = rng.gen_range(1, height - 1);
        let (size, horizontal) = if rng.gen::<f32>() < 0.7 {
            (rng.gen_range(0.0, max_width).max(min_width) as u32, true)
        } else {
            (rng.gen_range(0.0, max_height).max(min_height) as u32, false)
        };
        let wall = Wall(x, y, size, horizontal);

        if x + wall.width() > width || y + wall.height() > height {
            continue;
        }

        let intersects = walls
            .iter()
            .filter(|other| other.3 == wall.3)
            .any(|other| other.intersects(&wall));

        if intersects {
            continue;
        }

        walls.push(wall);

        if !valid_map(to_grid(&walls, width, height)) {
            walls.pop();
            continue;
        }
    }

    to_grid(&walls, width, height)
}

fn to_grid(walls: &[Wall], width: u32, height: u32) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = (0..width)
        .map(|_| (0..height).map(|_| false).collect())
        .collect();

    for wall in walls {
        for x in wall.x()..wall.x() + wall.width() {
            for y in wall.y()..wall.y() + wall.height() {
                grid[x as usize][y as usize] = true;
            }
        }
    }

    grid
}

fn valid_map(grid: Vec<Vec<bool>>) -> bool {
    let width = grid.len();
    let height = grid[0].len();
    let mut jump_test_left: Vec<(usize, usize)> = vec![];
    let mut jump_test_right: Vec<(usize, usize)> = vec![];

    for x in 0..width - 1 {
        for y in 0..height - 1 {
            let c = (
                grid[x][y],
                grid[x + 1][y],
                grid[x][y + 1],
                grid[x + 1][y + 1],
            );
            if c.0 && c.1 && c.2 && c.3 {
                // no 2x2 blocks
                return false;
            }
            if !c.0 && !c.1 && !c.2 && c.3 {
                jump_test_left.push((x, y));
            }
            if !c.0 && !c.1 && c.2 && !c.3 {
                jump_test_right.push((x + 1, y));
            }
        }
    }

    for x in 0..width {
        for y in 0..height {
            // no 1 wide gaps
            if x + 2 < width && grid[x][y] && !grid[x + 1][y] && grid[x + 2][y] {
                return false;
            }
            if y + 2 < height && grid[x][y] && !grid[x][y + 1] && grid[x][y + 2] {
                return false;
            }
        }
    }

    // reachability through jumping
    for (x, y) in jump_test_left {
        if !jumpable(x, y, &grid, (x as i32 - 6).max(0) as usize..=x) {
            return false;
        }
    }

    for (x, y) in jump_test_right {
        if !jumpable(x, y, &grid, x..=(x + 6).min(width - 1)) {
            return false;
        }
    }

    // connectivity
    let mut open = vec![];
    let mut closed = HashSet::new();

    for y in 0..height {
        for (x, row) in grid.iter().enumerate() {
            if !row[y] {
                open.push((x as i32, y as i32));
                closed.insert((x as i32, y as i32));
                break;
            }
        }
        if !open.is_empty() {
            break;
        }
    }

    while let Some((x, y)) = open.pop() {
        for neighbor in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)].iter() {
            let (nx, ny) = *neighbor;

            if nx >= 0 && !grid[nx as usize][ny as usize] && !closed.contains(&(nx, ny)) {
                open.push((nx, ny));
                closed.insert((nx, ny));
            }
        }
    }

    for y in 0..height {
        for (x, row) in grid.iter().enumerate() {
            if !row[y] && !closed.contains(&(x as i32, y as i32)) {
                return false;
            }
        }
    }

    true
}

fn jumpable(x: usize, y: usize, grid: &[Vec<bool>], range: RangeInclusive<usize>) -> bool {
    let mut lowest = 10000;
    for tx in range {
        if grid[tx][y] {
            break;
        }

        let mut bottom = y + 1 + (x as i32 - tx as i32).abs() as usize / 2;
        while bottom < grid[0].len() && !grid[tx][bottom] {
            bottom += 1;
        }
        lowest = lowest.min(bottom - 1 - y);
    }

    lowest <= 8
}

struct Wall(u32, u32, u32, bool);

impl Wall {
    fn intersects(&self, other: &Wall) -> bool {
        if self.3 {
            self.y() == other.y()
                && !(self.x() < other.x() && self.x() + self.width() < other.x()
                    || self.x() > other.x() + other.width()
                        && self.x() + self.width() > other.x() + other.width())
        } else {
            self.x() == other.x()
                && !(self.y() < other.y() && self.y() + self.height() < other.y()
                    || self.y() > other.y() + other.height()
                        && self.y() + self.height() > other.y() + other.height())
        }
    }

    fn x(&self) -> u32 {
        self.0
    }

    fn y(&self) -> u32 {
        self.1
    }

    fn width(&self) -> u32 {
        if self.3 {
            self.2
        } else {
            1
        }
    }

    fn height(&self) -> u32 {
        if self.3 {
            1
        } else {
            self.2
        }
    }
}
