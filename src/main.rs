use crate::map::Map;

mod map;
mod errors;
mod parser;

fn main() {
    let map = parser::parse();
    
}