pub mod map;

#[cfg(test)]
mod test {
  use crate::map::*;

  #[test]
  fn test_map_coord_error() {
    // let coord: Coord = Coord {
    //   lat: 100.0,
    //   long: 200.0,
    // };

    // match coord.validate() {
    //   Ok(_) => (),
    //   Err(e) => println!("{:?}", e)
    // };
  }

  #[test]
  fn test_map_cell() {
    let v = cell_pair(Cell { x: 0, y: 2, z: 0 });

    print!("{:?}", v);
  }
}
