use serde::Serialize;

use crate::domain::{color::Color, coord::Coord, group::GroupId};

#[derive(Debug, Serialize)]
pub struct GroupedStones {
    id: GroupId,
    color: Color,
    stones: Vec<Coord>,
}

impl GroupedStones {
    pub fn new(id: GroupId, color: Color, stones: Vec<Coord>) -> Self {
        Self { id, color, stones }
    }
}
