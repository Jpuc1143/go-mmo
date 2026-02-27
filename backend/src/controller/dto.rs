use serde::{Deserialize, Serialize};

use crate::domain::{
    color::Color,
    coord::Coord,
    group::{Group, GroupId},
    grouped_stones::GroupedStones,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupDto {
    id: GroupId,
    color: Color,
}

impl From<Group> for GroupDto {
    fn from(value: Group) -> Self {
        Self {
            id: value.id().unwrap(),
            color: value.color(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GroupedStonesDto {
    id: GroupId,
    color: Color,
    stones: Vec<Coord>,
}

impl From<GroupedStones> for GroupedStonesDto {
    fn from(value: GroupedStones) -> Self {
        Self {
            id: value.id(),
            color: value.color(),
            stones: value.stones().to_vec(),
        }
    }
}
