use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub fn generate_map(width: u8, height: u8) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    let num_walls = 16;
    let mut walls: Vec<Wall> = vec![];
    let min_width = (f64::from(width) * 0.3) as u8;
    let max_width = (f64::from(width) * 0.5) as u8;
    let min_height = (f64::from(height) * 0.3) as u8;
    let max_height = (f64::from(height) * 0.5) as u8;

    let x_distribution = Uniform::from(1..width - 1);
    let y_distribution = Uniform::from(1..height - 1);
    let width_distribution = Uniform::from(min_width..max_width);
    let height_distribution = Uniform::from(min_height..max_height);

    loop {
        walls.push(Wall::new(0, 0, width, 1));
        walls.push(Wall::new(0, height - 1, width, 1));
        walls.push(Wall::new(0, 0, 1, height));
        walls.push(Wall::new(width - 1, 0, 1, height));

        let mut tries = 0;
        while walls.len() < num_walls {
            if tries > 500 {
                break;
            }

            let x = x_distribution.sample(&mut rng);
            let y = y_distribution.sample(&mut rng);
            let (wall_width, wall_height) = if rng.gen::<f64>() < 0.7 {
                (width_distribution.sample(&mut rng), 1)
            } else {
                (1, height_distribution.sample(&mut rng))
            };
            let wall = Wall::new(x, y, wall_width, wall_height);

            if x + wall.width > width || y + wall.height > height {
                continue;
            }

            let intersects = walls
                .iter()
                .filter(|other| other.is_horizontal() == wall.is_horizontal())
                .any(|other| other.intersects(&wall));

            if intersects {
                tries += 1;
                continue;
            }

            walls.push(wall);

            if !valid_map(&to_grid(&walls, width, height)) {
                tries += 1;
                walls.pop();
                continue;
            }
        }

        if tries <= 500 {
            break;
        }

        walls.clear();
    }

    to_grid(&walls, width, height)
}

fn to_grid(walls: &[Wall], width: u8, height: u8) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; height as usize]; width as usize];

    for wall in walls {
        for x in wall.x..wall.x + wall.width {
            for y in wall.y..wall.y + wall.height {
                grid[x as usize][y as usize] = true;
            }
        }
    }

    grid
}

fn valid_map(grid: &[Vec<bool>]) -> bool {
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
            // ..
            // .#
            if !c.0 && !c.1 && !c.2 && c.3 {
                jump_test_left.push((x, y));
            }
            // ..
            // #.
            if !c.0 && !c.1 && c.2 && !c.3 {
                jump_test_right.push((x + 1, y));
            }
        }
    }

    // limit stalactites to 3
    if (0..width).filter(|&x| grid[x][1]).count() > 5 {
        return false;
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
        if !jumpable(x, y, grid, x.max(6) - 6..=x) {
            return false;
        }
    }

    for (x, y) in jump_test_right {
        if !jumpable(x, y, grid, x..=(x + 6).min(width - 1)) {
            return false;
        }
    }

    // connectivity
    let mut open = vec![];
    let mut closed = HashSet::new();

    for y in 0..height {
        for (x, row) in grid.iter().enumerate() {
            if !row[y] {
                open.push((x, y));
                closed.insert((x, y));
                break;
            }
        }
        if !open.is_empty() {
            break;
        }
    }

    // x and y can't be outside of grid, because of the border walls
    while let Some((x, y)) = open.pop() {
        for neighbor in &[(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
            let (nx, ny) = *neighbor;

            if !grid[nx][ny] && !closed.contains(&(nx, ny)) {
                open.push((nx, ny));
                closed.insert((nx, ny));
            }
        }
    }

    for y in 0..height {
        for (x, row) in grid.iter().enumerate() {
            if !row[y] && !closed.contains(&(x, y)) {
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
            return false;
        }

        let mut bottom = y + 1 + (x as i32 - tx as i32).abs() as usize / 2;
        while bottom < grid[0].len() && !grid[tx][bottom] {
            bottom += 1;
        }
        lowest = lowest.min(bottom - 1 - y);
    }

    lowest <= 8
}

#[derive(Debug)]
struct Wall {
    x: u8,
    y: u8,
    width: u8,
    height: u8,
}

impl Wall {
    fn new(x: u8, y: u8, width: u8, height: u8) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        let x_intersects = (self.x..self.x + self.width).contains(&other.x)
            || (other.x..other.x + other.width).contains(&self.x);
        let y_intersects = (self.y..self.y + self.height).contains(&other.y)
            || (other.y..other.y + other.height).contains(&self.y);
        x_intersects && y_intersects
    }

    fn is_horizontal(&self) -> bool {
        self.height == 1
    }
}
