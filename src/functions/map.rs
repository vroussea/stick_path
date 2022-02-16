#[derive(PartialEq, Eq)]
pub struct Cell {
    pub left: bool,
    pub right: bool,
    pub cell_char: char,
}

impl Cell {
    pub fn new((left, right): (bool, bool), character: char) -> Cell{
        return Cell {left: left, right: right, cell_char: character};
    }
}

pub struct Map {
    _height: u8,
    _width: u8,
    pub cells: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(height: u8, width: u8) -> Map {
        return Map {
            _height: height,
            _width: width,
            cells: Vec::new(),
        };
    }
}