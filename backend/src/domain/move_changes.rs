use super::{coord::Coord, group::GroupId};

pub struct MoveChanges {
    pub coord: Coord,
    pub assigned_group: GroupId,
    pub merged_groups: Vec<GroupId>,
}
