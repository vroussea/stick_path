use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Connector {
    left: bool,
    right: bool,
}

pub struct Map {
    height: i8,
    width: i8,
    map: Vec<Vec<Connector>>,
}

impl Map {
    pub fn parser() -> Map{
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let w = parse_input!(inputs[0], i32);
        let h = parse_input!(inputs[1], i32);
        for i in 0..h as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let line = input_line.trim_end().to_string();
            println!("{}", line);
        }
        
        return Map {height: 1, width: 2, map: Vec::new()};
    }
}