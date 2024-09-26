use macroquad::prelude::*;

mod cell;
mod maze;
mod recursive_backtracking_algorithm;

use cell::{Cell, CellType};
use maze::Maze;
use recursive_backtracking_algorithm::RecursiveBacktrackingAlgorithm;

#[macroquad::main("Maze Generator")]
async fn main() {
    let mut generator = RecursiveBacktrackingAlgorithm::new(21);  // Use an odd number for the size
    let mut finished = false;

    loop {
        clear_background(LIGHTGRAY);
        
        if !finished {
            finished = !generator.step();
        }
        
        generator.maze.draw_maze();
        
        next_frame().await
    }
}