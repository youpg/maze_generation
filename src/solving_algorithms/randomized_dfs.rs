use crate::maze::Maze;
use crate::cell::CellType;
use crate::maze_solver::MazeSolver;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct RandomizedDFS {
    maze: Maze,
    stack: Vec<(usize, usize)>,
    visited: Vec<bool>,
    end: (usize, usize),
    solved: bool,
    rng: rand::rngs::ThreadRng,
}

impl RandomizedDFS {
    pub fn new(maze: Maze) -> Self {
        let size = maze.size;
        let mut start = (0, 0);
        let mut end = (0, 0);

        for y in 0..size {
            for x in 0..size {
                match maze.get(x, y).unwrap().cell_type {
                    CellType::Start => start = (x, y),
                    CellType::End => end = (x, y),
                    _ => {}
                }
            }
        }

        RandomizedDFS {
            maze,
            stack: vec![start],
            visited: vec![false; size * size],
            end,
            solved: false,
            rng: thread_rng(),
        }
    }
}

impl MazeSolver for RandomizedDFS {
    fn step(&mut self) -> bool {
        if let Some(&(x, y)) = self.stack.last() {
            if (x, y) == self.end {
                self.solved = true;
                return false;
            }

            self.visited[y * self.maze.size + x] = true;

            let mut neighbors = vec![];
            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < self.maze.size as i32 && ny >= 0 && ny < self.maze.size as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if !self.visited[ny * self.maze.size + nx] &&
                        self.maze.get(nx, ny).unwrap().cell_type != CellType::Wall {
                            neighbors.push((nx, ny));
                        }
                }
            }

            if let Some(&next) = neighbors.choose(&mut self.rng) {
                self.stack.push(next);
            } else {
                self.stack.pop();
            }
            true
        } else {
            false
        }

        
    }

    fn solve_all(&mut self) {
        while self.step() {}
    }

    fn draw_solution(&self) {
        self.maze.draw_maze();

        let cell_size = macroquad::window::screen_width().min(macroquad::window::screen_height()) / self.maze.size as f32;
        for &(x, y) in &self.stack {
            macroquad::shapes::draw_circle(
                x as f32 * cell_size + cell_size / 2.0,
                y as f32 * cell_size + cell_size / 2.0,
                cell_size / 2.0,
                macroquad::color::GREEN,
            );
        }
    }
}