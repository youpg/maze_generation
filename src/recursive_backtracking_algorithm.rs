use crate::maze;
use crate::cell;
use crate::maze_generator;
use maze_generator::MazeGenerator;

use maze::Maze;
use cell::CellType;

use ::rand::thread_rng;
use ::rand::seq::SliceRandom;

pub struct RecursiveBacktrackingAlgorithm {
    pub maze: Maze,
    stack: Vec<(usize, usize)>,
    rng: ::rand::rngs::ThreadRng,
}

impl RecursiveBacktrackingAlgorithm {
    pub fn new(size: usize) -> Self {
        if size % 2 == 0 {
            panic!("Can't create maze with a even size, the size must be odd!");
        }
        let mut maze = Maze::new(size);
        maze.set(0, 0, CellType::Start);
        RecursiveBacktrackingAlgorithm {
            maze,
            stack: vec![(0, 0)],
            rng: thread_rng(),
        }
    }
}

impl MazeGenerator for RecursiveBacktrackingAlgorithm {
    fn step(&mut self) -> bool {
        if let Some(&(x, y)) = self.stack.last() {
            let mut neighbors = vec![];
            for (dx, dy) in &[(0, 2), (2, 0), (0, -2), (-2, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < self.maze.size as i32 && ny >= 0 && ny < self.maze.size as i32 
                   && self.maze.get(nx as usize, ny as usize).unwrap().cell_type == CellType::Wall {
                    neighbors.push((nx as usize, ny as usize));
                }
            }

            if !neighbors.is_empty() {
                let (nx, ny) = neighbors.choose(&mut self.rng).unwrap();
                self.maze.set((x + nx) / 2, (y + ny) / 2, CellType::Path);
                self.maze.set(*nx, *ny, CellType::Path);
                self.stack.push((*nx, *ny));
            } else {
                self.stack.pop();
            }
            true
        } else {
            self.maze.set(self.maze.size - 1, self.maze.size - 1, CellType::End);
            false
        }
    }

    fn generate_all(&mut self) {
        while self.step() {}
    }

    fn draw_maze(&self) {
        self.maze.draw_maze();
    }
}