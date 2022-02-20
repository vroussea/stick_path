#[derive(PartialEq, Eq, Debug, Clone)]
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


#[derive(Clone)]
pub struct Map {
    height: u8,
    width: u8,
    pub cells: Vec<Vec<Cell>>,
    pub answer: Vec<String>,
}

impl Map {
    pub fn new(height: u8, width: u8) -> Map {
        return Map {
            height: height,
            width: width,
            cells: Vec::new(),
            answer: Vec::new(),
        };
    }

    fn resolve_one_column(map: Map, mut current_columns: usize) -> Option<char> {
        for lines in 1..map.height as usize {
            if map.cells[lines][current_columns].cell_char != '|' {
                return Some(map.cells[lines][current_columns].cell_char);
            }
            else {
                if map.cells[lines][current_columns].left {
                    current_columns -= 1;
                }
                else if map.cells[lines][current_columns].right {
                    current_columns += 1;
                }
            }
        }
        return None;
    }

    pub fn resolve(self) -> Map {
        let mut map = self.clone();

        for columns in 0..map.width as usize {
            let mut current: String = String::from(map.cells[0][columns].cell_char);

            current.push(Map::resolve_one_column(map.clone(), columns).expect("how could that happen ?"));
            map.answer.push(current);
        }
        return map;
    }
}