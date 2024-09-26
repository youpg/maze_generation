use macroquad::prelude::*;

mod cell;
mod maze;
mod recursive_backtracking_algorithm;

use recursive_backtracking_algorithm::RecursiveBacktrackingAlgorithm;

#[macroquad::main("Maze Generator")]
async fn main() {
    let mut generator: RecursiveBacktrackingAlgorithm = RecursiveBacktrackingAlgorithm::new(299);  // Use an odd number for the size
    let mut finished: bool = false;
    let instant_generation: bool = false;

    if instant_generation {
        generator.generate_all();
        finished = true;
    }

    loop {
        clear_background(LIGHTGRAY);
        
        if !finished {
            finished = !generator.step();
        }
        
        generator.maze.draw_maze();
        
        next_frame().await
    }
}