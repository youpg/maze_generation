use crate::maze::Maze;

pub trait MazeSolver {
    fn step(&mut self) -> bool;
    fn solve_all(&mut self);
    fn draw_solution(&self);
    fn is_solved(&self) -> bool;
}