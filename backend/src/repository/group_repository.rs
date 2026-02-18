use super::super::domain::{
    coord::Coord,
    group::{Group, GroupId},
};

#[derive(Debug)]
pub struct GroupRepository {}

impl GroupRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn assigned_group_and_neighbors(&self, coord: Coord) -> (Option<GroupId>, Vec<Group>) {}

    pub fn merge_group(&self, target_group_id: GroupId, group: Group) {}

    pub fn upsert_group(&self, group: Group) {}

    pub fn delete_group(&self, group: Group) {}

    pub fn add_stone_to_group(&self, group_id: GroupId, coord: Coord) {}
}
