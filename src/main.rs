use macroquad::prelude::*;
use miniquad::window::{self, request_quit};

mod cell;
mod maze;

mod maze_generator;
use maze_generator::MazeGenerator;

mod maze_solver;
use maze_solver::MazeSolver;

// algorithms
mod algorithms;
use algorithms::recursive_backtracking::RecursiveBacktracking;
use algorithms::randomized_prims::RandomizedPrims;
use algorithms::randomized_kruskals::RandomizedKruskals;
use algorithms::aldous_broder::AldousBroder;
use algorithms::wilsons::Wilsons;

mod solving_algorithms;
use solving_algorithms::randomized_dfs::RandomizedDFS;


#[allow(dead_code)]
enum GeneratorAlgorithm {
    // implemented:
    RecursiveBacktracking,
    RandomizedPrims,
    RandomizedKruskals,
    AldousBroder,
    Wilsons,

    // not implemented yet (TODO):
    // RecursiveDivision
    // Hunt and kill
    // Ellers
    // BinaryTree
    // RandomizedDepthFirstSearch (DFS)
}

enum SolverAlgorithm {
    RandomizedDFS,

    // BFS,
    // Dijkstras,
    // AstarSearch,
}


#[macroquad::main("Maze Generator")]
async fn main() {
    window::set_window_size(1000, 1000);
    let maze_size: usize = 301; // odd number for maze_size
    let generator_algorithm: GeneratorAlgorithm = GeneratorAlgorithm::RecursiveBacktracking;


    let mut generator: Box<dyn MazeGenerator> = match generator_algorithm {
        GeneratorAlgorithm::RecursiveBacktracking => Box::new(RecursiveBacktracking::new(maze_size)),
        GeneratorAlgorithm::RandomizedPrims => Box::new(RandomizedPrims::new(maze_size)),
        GeneratorAlgorithm::RandomizedKruskals => Box::new(RandomizedKruskals::new(maze_size)),
        GeneratorAlgorithm::AldousBroder => Box::new(AldousBroder::new(maze_size)),
        GeneratorAlgorithm::Wilsons => Box::new(Wilsons::new(maze_size)),
    };

    let mut finished_generating: bool = false;
    let mut finished_solving: bool = false;
    let instant_generation: bool = true;
    let instant_solving: bool = true;
    
    if instant_generation {
        generator.generate_all();
        finished_generating = true;
    }

    let solver_algorithm: SolverAlgorithm = SolverAlgorithm::RandomizedDFS;
    let mut solver: Option<Box<dyn MazeSolver>> = None;

    loop {
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Escape) {
            request_quit();
        }
        
        if !finished_generating {
            finished_generating = !generator.step();
        } else if solver.is_none() {
            solver = Some(match solver_algorithm {
                SolverAlgorithm::RandomizedDFS => Box::new(RandomizedDFS::new(generator.get_maze())),
            });
            
            if instant_solving {
                solver.as_mut().unwrap().solve_all();
                finished_solving = true;
            }
        } else if !finished_solving {
            finished_solving = !solver.as_mut().unwrap().step();
        }
        
        if finished_generating {
            if let Some(solver) = &solver {
                solver.draw_solution();
            } else {
                generator.draw_maze();
            }
        } else {
            generator.draw_maze();
        }

        next_frame().await;
    }
}