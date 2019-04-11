use crate::cell::Cell;
use crate::map_generator;
use itertools::Itertools;
use std::iter::Iterator;

#[derive(Default)]
pub struct Map {
    pub width: u8,
    pub height: u8,
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

    fn coords_at(&self, x: f64, y: f64) -> (usize, usize) {
        (
            (x / 1920.0 * f64::from(self.width)) as usize,
            (y / 1080.0 * f64::from(self.height)) as usize,
        )
    }

    pub fn cell_at(&self, x: f64, y: f64) -> Cell {
        let (gx, gy) = self.coords_at(x, y);
        self.cell_at_grid(gx, gy)
    }

    fn cell_at_grid(&self, gx: usize, gy: usize) -> Cell {
        let (cw, ch) = (
            1920.0 / f64::from(self.width),
            1080.0 / f64::from(self.height),
        );
        Cell {
            x: gx as f64 * cw,
            y: gy as f64 * ch,
            w: cw,
            h: ch,
            state: self.cells[gx][gy],
        }
    }

    pub fn all_cells<'a>(&'a self) -> impl Iterator<Item = Cell> + 'a {
        (0..self.width as usize)
            .cartesian_product(0..self.height as usize)
            .filter(move |(gx, gy)| self.cells[*gx][*gy])
            .map(move |(gx, gy)| self.cell_at_grid(gx, gy))
    }

    pub fn cells_around<'a>(&'a self, x: f64, y: f64) -> impl Iterator<Item = Cell> + 'a {
        let (gx, gy) = self.coords_at(x, y);
        (gx.max(1) - 1..(gx + 2).min(self.width as usize))
            .cartesian_product(gy.max(1) - 1..(gy + 2).min(self.height as usize))
            .filter(move |(gx, gy)| self.cells[*gx][*gy])
            .map(move |(gx, gy)| self.cell_at_grid(gx, gy))
    }
}
