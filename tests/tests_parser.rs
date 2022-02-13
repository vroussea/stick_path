use super::map::Cell;

#[cfg(test)]
mod tests_map {
    #[test]
    fn convert_simple_line() {
        let line: &str = "|";
        let vec = convert_to_cells(line);
        assert_eq!(vec[0], Cell{left: false, right: false, cell_char: '|'});
    }
}