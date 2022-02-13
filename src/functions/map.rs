#[derive(PartialEq, Eq)]
pub struct Cell {
    pub left: bool,
    pub right: bool,
    pub cell_char: char,
}

pub struct Map {
    height: u8,
    width: u8,
    pub cells: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(height: u8, width: u8) -> Map {
        return Map {
            height: height,
            width: width,
            cells: Vec::new(),
        };
    }
}