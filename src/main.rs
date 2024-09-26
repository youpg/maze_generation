use macroquad::prelude::*;
use miniquad::window::{self, request_quit};

mod cell;
mod maze;
mod maze_generator;

// algorithms
mod recursive_backtracking;
mod randomized_prims;
mod randomized_kruskals;

use maze_generator::MazeGenerator;

// algorithms
use recursive_backtracking::RecursiveBacktracking;
use randomized_prims::RandomizedPrims;
use randomized_kruskals::RandomizedKruskals;

#[allow(dead_code)]
enum GeneratorAlgorithm {
    // implemented:
    RecursiveBacktracking,
    RandomizedPrims,

    // implementing:
    RandomizedKruskals,

    // not implemented yet (TODO):

    // Ellers
    // AldousBroder
    // Wilsons
    // RecursiveDivision
    // BinaryTree
    // RandomizedDepthFirstSearch (DFS)


}


#[macroquad::main("Maze Generator")]
async fn main() {
    window::set_window_size(1000, 1000);
    let maze_size: usize = 99; // odd number for maze_size
    let generator_algorithm: GeneratorAlgorithm = GeneratorAlgorithm::RandomizedPrims;


    let mut generator: Box<dyn MazeGenerator> = match generator_algorithm {
        GeneratorAlgorithm::RecursiveBacktracking => Box::new(RecursiveBacktracking::new(maze_size)),
        GeneratorAlgorithm::RandomizedPrims => Box::new(RandomizedPrims::new(maze_size)),
        GeneratorAlgorithm::RandomizedKruskals => Box::new(RandomizedKruskals::new(maze_size))

    };
    let mut finished: bool = false;
    let instant_generation: bool = false;
    
    if instant_generation {
        generator.generate_all();
        finished = true;
    }

    loop {
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Escape) {
            request_quit();
        }
        
        if !finished {
            finished = !generator.step();
        }
        
        generator.draw_maze();
        next_frame().await
    }
}