use crate::maze;
use crate::cell;
use crate::maze_generator;
use maze_generator::MazeGenerator;

use maze::Maze;
use cell::{Cell, CellType};

pub struct RandomizedPrimsAlgorithm {
    pub maze: Maze,
}

impl RandomizedPrimsAlgorithm {
    pub fn new(size: usize) -> Self {
        let mut maze: Maze = Maze::new(size);
        maze.set(0, 0, CellType::Start);
        RandomizedPrimsAlgorithm {
            maze,
        }
    }
}

impl MazeGenerator for RandomizedPrimsAlgorithm {


    fn step(&mut self) -> bool {
        true
    }
    
    fn generate_all(&mut self) {
        while self.step() {}
    }

    fn draw_maze(&self) {
        self.maze.draw_maze();
    }
}