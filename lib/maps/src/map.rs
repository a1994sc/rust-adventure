use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct Coord {
  #[validate(range(min = -90.0, max = 90.0))]
  pub lat: f32,
  #[validate(range(min = -180.0, max = 180.0))]
  pub long: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Cell {
  pub x: u32,
  pub y: u32,
  pub z: u32,
}

const LIMIT: u32 = 2500;

pub fn cell_pair(cell: Cell) -> u32 {
  cell.x + (cell.y * LIMIT) + (cell.z * LIMIT * LIMIT)
}

// pub fn cell_separate(cell: Cell) -> u32 {
//   cell.x + (cell.y * LIMIT) + (cell.z * LIMIT * LIMIT)
// }
