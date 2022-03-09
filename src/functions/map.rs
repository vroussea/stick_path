use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Cell {
    pub left: bool,
    pub right: bool,
    pub cell_char: char,
}

impl Cell {
    pub fn new((left, right): (bool, bool), character: char) -> Cell {
        return Cell {
            left: left,
            right: right,
            cell_char: character,
        };
    }
}

#[derive(Clone)]
pub struct Map {
    pub height: u8,
    pub width: u8,
    pub cells: Vec<Vec<Cell>>,
    pub answer: Vec<String>,
}

impl Map {
    pub fn new(width: u8, height: u8) -> Map {
        return Map {
            height,
            width,
            cells: Vec::new(),
            answer: Vec::new(),
        };
    }

    fn resolve_one_column(map: &Map, mut current_columns: usize) -> Option<char> {
        for lines in 1..map.height as usize {
            if map.cells[lines][current_columns].cell_char != '|' {
                return Some(map.cells[lines][current_columns].cell_char);
            } else {
                if map.cells[lines][current_columns].left {
                    current_columns -= 1;
                } else if map.cells[lines][current_columns].right {
                    current_columns += 1;
                }
            }
        }
        return None;
    }

    pub fn resolve(&mut self) -> &Self {
        for columns in 0..self.width as usize {
            let mut current: String = String::from(self.cells[0][columns].cell_char);
            current.push(Map::resolve_one_column(self, columns).expect("Error in width/heigh"));
            self.answer.push(current);
        }
        return self;
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        if self.answer.len() > 0 {
            result.push_str(&self.answer[0]);
            if self.answer.len() > 1 {
                for answer in &self.answer[1..] {
                    result.push('\n');
                    result.push_str(&answer);
                }
            } else {
                result.push_str("\n");
            }
        }
        write!(f, "{}", result)
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("{} {}\n", self.width, self.height));
        for line in &self.cells[..] {
            for cell in line {
                result.push_str(&format!("{:?} ", cell));
            }
            result.push_str("\n");
        }
        for answer in &self.answer[..] {
            result.push_str(&answer);
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests_map {
    use super::*;
    #[test]
    fn small_map_easy() {
        let mut map = Map::new(3, 3);
        map.cells.push(
            [
                Cell::new((false, false), 'a'),
                Cell::new((false, false), 'b'),
                Cell::new((false, false), 'c'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '1'),
                Cell::new((false, false), '2'),
                Cell::new((false, false), '3'),
            ]
            .to_vec(),
        );
        let map = map.resolve();

        assert_eq!(map.answer, ["a1", "b2", "c3"].to_vec());
    }

    #[test]
    fn small_map_hard() {
        let mut map = Map::new(3, 3);
        map.cells.push(
            [
                Cell::new((false, false), 'a'),
                Cell::new((false, false), 'b'),
                Cell::new((false, false), 'c'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, true), '|'),
                Cell::new((true, false), '|'),
                Cell::new((false, false), '|'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '1'),
                Cell::new((false, false), '2'),
                Cell::new((false, false), '3'),
            ]
            .to_vec(),
        );
        let map = map.resolve();

        assert_eq!(map.answer, ["a2", "b1", "c3"].to_vec());
    }

    #[test]
    fn medium_map_easy() {
        let mut map = Map::new(4, 4);
        map.cells.push(
            [
                Cell::new((false, false), 'a'),
                Cell::new((false, false), 'b'),
                Cell::new((false, false), 'c'),
                Cell::new((false, false), 'd'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '1'),
                Cell::new((false, false), '2'),
                Cell::new((false, false), '3'),
                Cell::new((false, false), '4'),
            ]
            .to_vec(),
        );
        let map = map.resolve();

        assert_eq!(map.answer, ["a1", "b2", "c3", "d4"].to_vec());
    }

    #[test]
    fn medium_map_hard() {
        let mut map = Map::new(4, 4);
        map.cells.push(
            [
                Cell::new((false, false), 'a'),
                Cell::new((false, false), 'b'),
                Cell::new((false, false), 'c'),
                Cell::new((false, false), 'd'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, true), '|'),
                Cell::new((true, false), '|'),
                Cell::new((false, true), '|'),
                Cell::new((true, false), '|'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '|'),
                Cell::new((false, true), '|'),
                Cell::new((true, false), '|'),
                Cell::new((false, false), '|'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '1'),
                Cell::new((false, false), '2'),
                Cell::new((false, false), '3'),
                Cell::new((false, false), '4'),
            ]
            .to_vec(),
        );
        let map = map.resolve();

        assert_eq!(map.answer, ["a3", "b1", "c4", "d2"].to_vec());
    }

    #[test]
    fn example_one() {
        let mut map = Map::new(3, 7);
        map.cells.push(
            [
                Cell::new((false, false), 'a'),
                Cell::new((false, false), 'b'),
                Cell::new((false, false), 'c'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, true), '|'),
                Cell::new((true, false), '|'),
                Cell::new((false, false), '|'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '|'),
                Cell::new((false, true), '|'),
                Cell::new((true, false), '|'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '|'),
                Cell::new((false, true), '|'),
                Cell::new((true, false), '|'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
                Cell::new((false, false), '|'),
            ]
            .to_vec(),
        );
        map.cells.push(
            [
                Cell::new((false, false), '1'),
                Cell::new((false, false), '2'),
                Cell::new((false, false), '3'),
            ]
            .to_vec(),
        );
        let map = map.resolve();

        assert_eq!(map.answer, ["a2", "b1", "c3"].to_vec());
    }
}
