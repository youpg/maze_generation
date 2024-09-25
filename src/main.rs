mod stack;
use stack::Stack;

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Wall,
    Path,
    Start, 
    End,
}

#[derive(Clone)]
struct Maze {
    size: usize,
    grid: Vec<Cell>,
}

impl Maze {
    fn new(size: usize) -> Self {
        let grid = vec![Cell::Wall; size * size];
        Maze { size, grid }
    }

    fn get(&self, index: usize) -> Option<&Cell> {
        self.grid.get(index)
    }

    fn set(&mut self, index: usize, cell: Cell) {
        if index < self.size * self.size {
            self.grid[index] = cell;
        }
    }
}

fn main() {
    let mut maze: Maze = Maze::new(3);
    let starting_cell = 0;
    maze.set(starting_cell, Cell::Start);
    
    let mut maze_route: Stack<usize> = Stack::new();
    maze_route.push(starting_cell);

    println!("{:?}", get_neighbors(&maze.size, 8));
}

fn get_neighbors(maze_size: &usize, cell_index: usize) -> [Option<usize>; 4] {
    let mut neighbors: [Option<usize>; 4] = [None; 4];
    if cell_index < maze_size*maze_size {
        // right neighbor index 0
        if (cell_index + 1) % *maze_size != 0 {
            neighbors[0] = Some(cell_index + 1);
        }
    
        // top neighbor index 1
        if cell_index >= *maze_size {
            neighbors[1] = Some(cell_index - maze_size);
        }
    
        // left neighbor index 2
        if cell_index % *maze_size != 0 {
            neighbors[2] = Some(cell_index - 1);
        }
    
        // bottom neighbor index 3
        if cell_index < maze_size * (maze_size - 1) {
            neighbors[3] = Some(cell_index + maze_size)
        }
    
    }
    neighbors // No neighbors [None; 4] invalid cell_index
}
