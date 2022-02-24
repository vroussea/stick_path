#[cfg(test)]
mod tests_map {
    use stick_path::functions::map::{Map, Cell};
    #[test]
    fn small_map() {
        let mut map = Map::new(3, 1);
        
        map.cells.push([Cell::new((false, false), 'a')].to_vec());
        map.cells.push([Cell::new((false, false), '|')].to_vec());
        map.cells.push([Cell::new((false, false), '1')].to_vec());
        println!("cells: {:?}", map.cells);
        let map =  map.resolve();

        assert_eq!(map.answer, ["a1"].to_vec());
    }
}