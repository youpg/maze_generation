use crate::maze::Maze;

pub trait MazeGenerator {
    fn step(&mut self) -> bool;
    fn generate_all(&mut self);
    fn draw_maze(&self);
    fn get_maze(&self) -> Maze;
}