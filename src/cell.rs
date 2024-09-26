#[derive(Clone, Debug, PartialEq)]
pub enum CellType {
    Wall,
    Path,
    Start,
    End,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub cell_type: CellType,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        Cell { cell_type }
    }
}