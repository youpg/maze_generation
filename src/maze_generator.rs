pub trait MazeGenerator {
    // fn new(maze_size: usize) -> Self;
    fn step(&mut self) -> bool;
    fn generate_all(&mut self);
    fn draw_maze(&self);
}