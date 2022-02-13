use crate::errors;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>()?)
}

struct Cell {
    left: bool,
    right: bool,
    cell_char: char,
}

pub struct Map {
    height: u8,
    width: u8,
    map: Vec<Vec<Cell>>,
}

fn read_line() -> Result<String, errors::CustomError>{
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    return Ok(input_line);
}

fn get_size() -> Result<(u8, u8), errors::CustomError> {
    let input_line = read_line()?;
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], u8);
    let height = parse_input!(inputs[1], u8);
    return Ok((width, height));
}

impl Map {
    pub fn parser() -> Result<Map, errors::CustomError> {
        let (width, height) = get_size()?;
        let map = Map::new(height, width);
        for i in 0..height as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line)?;
            let line = input_line.trim_end().to_string();
            println!("{}", line);
        }
        
        return Ok(Map {height: 1, width: 2, map: Vec::new()});
    }

    fn new(height: u8, width: u8) -> Map {
        return Map {
            height: height,
            width: width,
            map: Vec::new(),
        };
    }
}