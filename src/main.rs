use core::panic;

use macroquad::prelude::*;

mod cell;
mod maze;
mod recursive_backtracking_algorithm;
mod randomized_prims_algorithm;
mod maze_generator;

use recursive_backtracking_algorithm::RecursiveBacktrackingAlgorithm;
use randomized_prims_algorithm::RandomizedPrimsAlgorithm;
use maze_generator::MazeGenerator;

enum GeneratorAlgorithm {
    RecursiveBacktrackingAlgorithm,
    RandomizedPrimsAlgorithm,
}


#[macroquad::main("Maze Generator")]
async fn main() {
    let maze_size: usize = 99; // odd number for maze_size
    let generator_algorithm: GeneratorAlgorithm = GeneratorAlgorithm::RecursiveBacktrackingAlgorithm;


    let mut generator: Box<dyn MazeGenerator> = match generator_algorithm {
        GeneratorAlgorithm::RecursiveBacktrackingAlgorithm => Box::new(RecursiveBacktrackingAlgorithm::new(maze_size)),
        GeneratorAlgorithm::RandomizedPrimsAlgorithm => Box::new(RandomizedPrimsAlgorithm::new(maze_size)),
    };
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
        
        generator.draw_maze();
        next_frame().await
    }
}