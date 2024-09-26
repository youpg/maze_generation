use crate::maze;
use crate::cell;
use crate::maze_generator;

use maze::Maze;
use cell::CellType;
use maze_generator::MazeGenerator;
use ::rand::thread_rng;
use rand::Rng;

pub struct RandomizedPrimsAlgorithm {
    pub maze: Maze,
    pub frontier: Vec<(usize, usize)>,
    rng: ::rand::rngs::ThreadRng,
}

impl RandomizedPrimsAlgorithm {
    pub fn new(size: usize) -> Self {
        if size % 2 == 0 {
            panic!("Can't create a maze with a even size, the size must be odd!");
        }
        let mut maze: Maze = Maze::new(size + 2); // Increase size by 2 for walls
        
        // Fill the entire maze with walls
        for x in 0..size {
            for y in 0..size {
                maze.set(x, y, CellType::Wall);
            }
        }
        
        let (start_x, start_y) = (1, 1); // Start at (1,1) instead of (0,0)
        maze.set(start_x, start_y, CellType::Path);
        
        let mut frontier = Vec::new();
        Self::add_frontier(&mut maze, &mut frontier, start_x, start_y);

        RandomizedPrimsAlgorithm {
            maze,
            frontier,
            rng: thread_rng(),
        }
    }

    fn add_frontier(maze: &mut Maze, frontier: &mut Vec<(usize, usize)>, x: usize, y: usize) {
        let directions = vec![(0, 2), (2, 0), (0, -2), (-2, 0)];
        for (dx, dy) in directions {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if nx > 0 && nx < maze.size - 1 && ny > 0 && ny < maze.size - 1 && maze.get(nx, ny).unwrap().cell_type == CellType::Wall {
                if !frontier.contains(&(nx, ny)) {
                    frontier.push((nx, ny));
                }
            }
        }
    }

    fn connect_to_maze(&mut self, x: usize, y: usize) -> bool {
        let directions = vec![(0, -2), (-2, 0), (0, 2), (2, 0)];
        
        let mut valid_neighbors = Vec::new();

        for (dx, dy) in directions {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if nx > 0 && nx < self.maze.size - 1 && ny > 0 && ny < self.maze.size - 1 {
                let neighbor = self.maze.get(nx, ny).unwrap();
                if neighbor.cell_type == CellType::Path {
                    valid_neighbors.push((nx, ny));
                }
            }
        }

        if !valid_neighbors.is_empty() {
            let (nx, ny) = valid_neighbors[self.rng.gen_range(0..valid_neighbors.len())];
            self.maze.set(x, y, CellType::Path);
            let wall_x = (x + nx) / 2;
            let wall_y = (y + ny) / 2;
            self.maze.set(wall_x, wall_y, CellType::Path);
            true
        } else {
            false
        }
    }
}

impl MazeGenerator for RandomizedPrimsAlgorithm {
    fn step(&mut self) -> bool {
        if self.frontier.is_empty() {
            self.maze.set(self.maze.size - 2, self.maze.size - 1, CellType::End);
            self.maze.set(1, 0, CellType::Start);
            println!("Maze generation is finished");
            return false;
        }

        let rand_index = self.rng.gen_range(0..self.frontier.len());
        let (x, y) = self.frontier.swap_remove(rand_index);

        if self.connect_to_maze(x, y) {
            Self::add_frontier(&mut self.maze, &mut self.frontier, x, y);
        }

        true
    }
    
    fn generate_all(&mut self) {
        while self.step() {}
    }

    fn draw_maze(&self) {
        self.maze.draw_maze();
    }
}