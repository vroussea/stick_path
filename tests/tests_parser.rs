#[cfg(test)]
mod tests_map {
    use stick_path::functions::map;

    #[test]
    fn convert_simple_line() {
        let line: &str = "|";
        let vec = convert_to_cells(line);
        assert_eq!(vec[0], map::Cell{left: false, right: false, cell_char: '|'});
    }

    fn convert_three_characters_line() {
        let line: &str = "| | |";
        let vec = convert_to_cells(line);
        assert_eq!(vec[0], map::Cell{left: false, right: false, cell_char: '|'});
        assert_eq!(vec[1], map::Cell{left: false, right: false, cell_char: '|'});
        assert_eq!(vec[2], map::Cell{left: false, right: false, cell_char: '|'});
    }

    fn convert_one_bridge() {
        let line: &str = "| |--|";
        let vec = convert_to_cells(line);
        assert_eq!(vec[0], map::Cell{left: false, right: false, cell_char: '|'});
        assert_eq!(vec[1], map::Cell{left: false, right: true, cell_char: '|'});
        assert_eq!(vec[2], map::Cell{left: true, right: false, cell_char: '|'});
    }

    fn convert_two_bridges() {
        let line: &str = "|--|--|";
        let vec = convert_to_cells(line);
        assert_eq!(vec[0], map::Cell{left: false, right: true, cell_char: '|'});
        assert_eq!(vec[1], map::Cell{left: true, right: true, cell_char: '|'});
        assert_eq!(vec[2], map::Cell{left: true, right: false, cell_char: '|'});
    }

    fn first_line() {
        let line: &str = "? A n ,";
        let vec = convert_to_cells(line);
        assert_eq!(vec[0], map::Cell{left: false, right: false, cell_char: '?'});
        assert_eq!(vec[1], map::Cell{left: false, right: false, cell_char: 'A'});
        assert_eq!(vec[2], map::Cell{left: false, right: false, cell_char: 'n'});
        assert_eq!(vec[3], map::Cell{left: false, right: false, cell_char: ','});
    }

    fn last_line() {
        let line: &str = "1 A $ ,";
        let vec = convert_to_cells(line);
        assert_eq!(vec[0], map::Cell{left: false, right: false, cell_char: '1'});
        assert_eq!(vec[1], map::Cell{left: false, right: false, cell_char: 'A'});
        assert_eq!(vec[2], map::Cell{left: false, right: false, cell_char: '$'});
        assert_eq!(vec[3], map::Cell{left: false, right: false, cell_char: ','});
    }
}