use crate::functions::errors;
use std::io;
use crate::functions::map::{Map, Cell};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>()?)
}

pub fn parse() -> Result<Map, errors::CustomError> {
    let (width, height) = read_size()?;
    let mut map = Map::new(height, width);
    for _ in 0..height {
        map.cells.push(read_line()?);
    }
    
    return Ok(map);
}


fn read_input() -> Result<String, errors::CustomError>{
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    return Ok(input_line);
}

fn read_line() -> Result<Vec<Cell>, errors::CustomError>{
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    let line = input_line.trim_end().to_string();
    println!("{}", line);
    return Ok(Vec::new());
}

fn read_size() -> Result<(u8, u8), errors::CustomError> {
    let input_line = read_input()?;
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], u8);
    let height = parse_input!(inputs[1], u8);
    return Ok((width, height));
}