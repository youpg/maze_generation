use crate::maze;
use crate::cell;
use crate::maze_generator;

use maze::Maze;
use cell::CellType;
use maze_generator::MazeGenerator;

use ::rand::thread_rng;
use rand::seq::SliceRandom;



pub struct RandomizedKruskals {
    maze: Maze,
    walls: Vec<((usize, usize), (usize, usize))>,
    disjoint_set: Vec<usize>,
}

impl RandomizedKruskals {
    pub fn new(size: usize) -> Self {
        if size % 2 == 0 {
            panic!("Can't create maze with even size, the ssize must be odd!");
        }
        let mut maze = Maze::new(size); // add 2 to the size for walls

        // initialise all cells as walls 
        for x in 0..size {
            for y in 0..size {
                maze.set(x, y, CellType::Wall);
            }
        }

        for y in (1..size).step_by(2) {
            for x in (1..size).step_by(2) {
                maze.set(x, y, CellType::Path);
            }
        }

        let mut walls = Vec::new();
        for y in (1..size).step_by(2) {
            for x in (1..size).step_by(2) {
                if x + 2 < size {
                    walls.push(((x, y), (x+2, y)));
                }
                if y + 2 < size {
                    walls.push(((x, y), (x, y+2)));
                }
            }
        }
        
        walls.shuffle(&mut thread_rng());
        let disjoint_set: Vec<usize> = (0..size*size).collect();

        RandomizedKruskals {
            maze,
            walls,
            disjoint_set,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.disjoint_set[x] != x {
            self.disjoint_set[x] = self.find(self.disjoint_set[x]);
        }
        self.disjoint_set[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.disjoint_set[root_x] = root_y;
        }
    }
}

impl MazeGenerator for RandomizedKruskals {
    fn step(&mut self) -> bool {
        if let Some(wall) = self.walls.pop() {
            let ((x1, y1), (x2, y2)) = wall;
            let cell1 = y1 * self.maze.size + x1;
            let cell2 = y2 * self.maze.size + x2;

            if self.find(cell1) != self.find(cell2) {
                self.union(cell1, cell2);
                self.maze.set((x1+x2)/2, (y1+y2)/2, CellType::Path);
            }
            true
        } else {
            self.maze.set(1, 0, CellType::Start);
            self.maze.set(self.maze.size - 2, self.maze.size - 1, CellType::End);
            println!("Maze generation is finished");
            false
        }
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