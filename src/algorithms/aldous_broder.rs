use crate::maze;
use crate::cell;
use crate::maze_generator;

use maze_generator::MazeGenerator;
use maze::Maze;
use cell::CellType;

use ::rand::thread_rng;
use rand::Rng;

pub struct AldousBroder {
    maze: Maze,
    current: (usize, usize),
    unvisited: usize,
    rng: ::rand::rngs::ThreadRng,
}


impl AldousBroder {
    pub fn new(size: usize) -> Self {
        if size % 2 == 0 {
            panic!("Can't create maze with a even size, the size must be odd!");
        }
        let mut maze = Maze::new(size);

        for x in 0..size {
            for y in 0..size {
                maze.set(x, y, CellType::Wall);
            }
        }

        let unvisited = ((size - 1) / 2).pow(2);
        let current = (1, 1);
        maze.set(current.0, current.1, CellType::Path);

        AldousBroder {
            maze,
            current,
            unvisited: unvisited - 1,
            rng: thread_rng(),
        }
    }

    fn get_neighbors(&self) -> Vec<(usize, usize)> {
        let (x, y) = self.current;
        let size = self.maze.size;
        let mut neighbors = Vec::new();

        if x >= 3 { neighbors.push((x - 2, y)); }
        if x <= size - 3 { neighbors.push((x + 2, y)); }
        if y >= 3 { neighbors.push((x, y - 2)); }
        if y <= size - 3 { neighbors.push((x, y + 2)); }

        neighbors
    }

}

impl MazeGenerator for AldousBroder {
    fn step(&mut self) -> bool {
        if self.unvisited == 0 {
            self.maze.set(self.maze.size - 2, self.maze.size - 1, CellType::End);
            self.maze.set(1, 0, CellType::Start);
            println!("Maze generation is finished");
            return false;
        }

        let neighbors = self.get_neighbors();
        let next = neighbors[self.rng.gen_range(0..neighbors.len())];

        if self.maze.get(next.0, next.1).unwrap().cell_type == CellType::Wall {
            let mid_x = (self.current.0 + next.0) / 2;
            let mid_y = (self.current.1 + next.1) / 2;
            self.maze.set(mid_x, mid_y, CellType::Path);
            self.maze.set(next.0, next.1, CellType::Path);
            self.unvisited -= 1;
        }

        self.current = next;
        true
    }

    fn generate_all(&mut self) {
        while self.step() {}
    }

    fn draw_maze(&self) {
        self.maze.draw_maze();
    }

    fn get_maze(&self) -> Maze {
        self.maze.clone()
    }
}