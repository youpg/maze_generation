use macroquad::prelude::*;

use crate::cell;
use cell::{Cell, CellType};

#[derive(Debug, Clone)]
pub struct Maze {
    pub size: usize,
    pub grid: Vec<Cell>,
}

impl Maze {
    pub fn new(size: usize) -> Self {
        let grid = vec![Cell::new(CellType::Wall); size * size];
        Maze { size, grid }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.size && y < self.size {
            Some(&self.grid[y * self.size + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell_type: CellType) {
        if x < self.size && y < self.size {
            self.grid[y * self.size + x].cell_type = cell_type;
        }
    }

    pub fn draw_maze(&self) {
        let cell_size = screen_width().min(screen_height()) / self.size as f32;
        for y in 0..self.size {
            for x in 0..self.size {
                let cell = self.get(x, y).unwrap();
                let color = match cell.cell_type {
                    CellType::Wall => BLACK,
                    CellType::Path => WHITE,
                    CellType::Start => BLUE,
                    CellType::End => RED,
                };
                draw_rectangle(x as f32 * cell_size, y as f32 * cell_size, cell_size, cell_size, color);
            }
        }
    }
}