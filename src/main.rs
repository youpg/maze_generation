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

    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.grid.get(y * self.size + x)
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.size && y < self.size {
            self.grid[y * self.size + x ] = cell;
        }
    }
}

fn main() {
    let mut maze: Maze = Maze::new(10);
    let starting_cell = (0, 0);
    maze.set(starting_cell.0, starting_cell.1, Cell::Start);
    
    let mut maze_route: Stack<(usize, usize)> = Stack::new();
    maze_route.push(starting_cell);

    


}

fn print_maze(maze: &Maze) {
    for i in 0..maze.size {
        for j in 0..maze.size {
            let cell_data: &str = match maze.get(i, j) {
                Some(n) => {
                    if *n == Cell::Wall {
                        "🧱"
                    } else if *n == Cell::Start {
                        "⭕"
                    } else {
                        "ERROR"
                    }
                }
                None => { return; }
            };
            print!("{}", cell_data)
        }
        println!("");
    }
}