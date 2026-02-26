use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }
}

impl From<Coord> for (i64, i64) {
    fn from(value: Coord) -> Self {
        (value.x, value.y)
    }
}
