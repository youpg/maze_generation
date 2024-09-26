use crate::maze;
use crate::cell;
use crate::maze_generator;

use maze::Maze;
use cell::CellType;
use maze_generator::MazeGenerator;

use ::rand::thread_rng;



pub struct RandomizedKruskals {
    pub maze: Maze,
    
    rng: ::rand::rngs::ThreadRng,
}

impl RandomizedKruskals {
    pub fn new(size: usize) -> Self {
        if size % 2 == 0 {
            panic!("Can't create maze with even size, the ssize must be odd!");
        }
        let mut maze = Maze::new(size + 2); // add 2 to the size for walls

        for x in 0..size {
            for y in 0..size {
                maze.set(x, y, CellType::Wall);
            }
        }

        let (start_x, start_y) = (1, 1);
        maze.set(start_x, start_y, CellType::Path);

        RandomizedKruskals {
            maze,
            rng: thread_rng(),
        }
    }
}

impl MazeGenerator for RandomizedKruskals {
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