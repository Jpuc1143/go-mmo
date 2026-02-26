use serde::Serialize;

use crate::{
    controller::dto::GroupDto,
    domain::{coord::Coord, group::GroupId, stone_placed_changes::StonePlacedChanges},
};

#[derive(Debug, Serialize)]
pub enum GameServerMessage {
    BoardData,
    StonePlaced {
        coord: Coord,
        assigned_group: GroupDto,
        captured_groups_ids: Vec<GroupId>,
        merged_groups_ids: Vec<GroupId>,
    },
}

impl From<StonePlacedChanges> for GameServerMessage {
    fn from(value: StonePlacedChanges) -> Self {
        Self::StonePlaced {
            coord: value.coord,
            assigned_group: value.assigned_group.into(),
            captured_groups_ids: value.captured_groups_ids,
            merged_groups_ids: value.merged_groups_ids,
        }
    }
}
