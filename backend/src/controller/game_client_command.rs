use serde::Deserialize;

use crate::domain::{color::Color, coord::Coord};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum GameClientCommand {
    RequestConfiguration,
    PlaceStone { coord: Coord, color: Color },
}
