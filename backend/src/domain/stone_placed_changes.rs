use crate::domain::group::Group;

use super::{coord::Coord, group::GroupId};

pub struct StonePlacedChanges {
    pub coord: Coord,
    pub assigned_group: Group,
    pub captured_groups_ids: Vec<GroupId>,
    pub merged_groups_ids: Vec<GroupId>,
}
