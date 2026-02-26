use serde::Deserialize;

use crate::domain::{color::Color, coord::Coord};

#[derive(Debug, Deserialize)]
pub enum GameClientCommand {
    PlaceStone { coord: Coord, color: Color },
}
