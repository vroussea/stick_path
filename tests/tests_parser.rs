#[cfg(test)]
mod tests_map {
    use stick_path::functions::map;

    #[test]
    fn convert_simple_line() {
        let line: &str = "|";
        let vec = convert_to_cells(line);
        assert_eq!(vec[0], map::Cell{left: false, right: false, cell_char: '|'});
    }
}