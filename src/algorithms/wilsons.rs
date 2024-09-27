use crate::maze;
use crate::cell;
use crate::maze_generator;
use maze_generator::MazeGenerator;

use maze::Maze;
use cell::CellType;

use ::rand::thread_rng;
use ::rand::seq::SliceRandom;

pub struct Wilsons {
    maze: Maze,
    unvisited: Vec<(usize, usize)>,
    current_path: Vec<(usize, usize)>,
    rng: ::rand::rngs::ThreadRng,
    current: (usize, usize),
    state: WilsonState,
}

enum WilsonState {
    PickStart,
    RandomWalk,
    AddPath,
}

impl Wilsons {
    pub fn new(size: usize) -> Self {
        if size % 2 == 0 {
            panic!("Can't create maze with an even size, the size must be odd!");
        }
        let mut maze = Maze::new(size);
        let mut unvisited: Vec<(usize, usize)> = Vec::new();

        for x in (1..size).step_by(2) {
            for y in (1..size).step_by(2) {
                maze.set(x, y, CellType::Wall);
                unvisited.push((x, y));
            }
        }

        let start = *unvisited.choose(&mut thread_rng()).unwrap();
        maze.set(start.0, start.1, CellType::Path);
        unvisited.retain(|&cell| cell != start);

        maze.set(1, 0, CellType::Start);
        maze.set(size - 2, size - 1, CellType::End);

        Wilsons {
            maze,
            unvisited,
            current_path: Vec::new(),
            rng: thread_rng(),
            current: (0, 0),
            state: WilsonState::PickStart,
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
        
        for (dx, dy) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 1 && nx < self.maze.size as i32 - 1 && ny >= 1 && ny < self.maze.size as i32 - 1 {
                neighbors.push((nx as usize, ny as usize));
            }
        }
        neighbors
    }

    fn draw_path(&mut self, cell_type: &CellType) {
        for window in self.current_path.windows(2) {
            let (x1, y1) = window[0];
            let (x2, y2) = window[1];
            
            let dx = (x2 as i32 - x1 as i32).signum();
            let dy = (y2 as i32 - y1 as i32).signum();
            let mut x = x1 as i32;
            let mut y = y1 as i32;
            
            while (x, y) != (x2 as i32, y2 as i32) {
                self.maze.set(x as usize, y as usize, cell_type.clone());
                x += dx;
                y += dy;
            }
            self.maze.set(x2, y2, cell_type.clone());
        }
    }
}

impl MazeGenerator for Wilsons {
    fn step(&mut self) -> bool {
        match self.state {
            WilsonState::PickStart => {
                if self.unvisited.is_empty() {
                    return false;
                }
                self.current = *self.unvisited.choose(&mut self.rng).unwrap();
                self.current_path.clear();
                self.current_path.push(self.current);
                self.state = WilsonState::RandomWalk;
            }
            WilsonState::RandomWalk => {
                let neighbors = self.get_neighbors(self.current.0, self.current.1);
                let next = *neighbors.choose(&mut self.rng).unwrap();
                
                if self.maze.get(next.0, next.1).unwrap().cell_type == CellType::Path {
                    self.current_path.push(next);
                    self.state = WilsonState::AddPath;
                } else {
                    if let Some(loop_start) = self.current_path.iter().position(|&x| x == next) {
                        self.draw_path(&CellType::Wall);
                        self.current_path.truncate(loop_start + 1);
                    } else {
                        self.current_path.push(next);
                    }
                    self.current = next;
                }
                
                self.draw_path(&CellType::Start);
            }
            WilsonState::AddPath => {
                self.draw_path(&CellType::Path);
                for &(x, y) in &self.current_path {
                    self.unvisited.retain(|&cell| cell != (x, y));
                }
                self.state = WilsonState::PickStart;
            }
        }
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