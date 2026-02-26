use std::{collections::HashSet, fmt::Debug};

use diesel::dsl::exists;
use diesel::prelude::*;

use super::super::domain::{
    coord::Coord,
    group::{Group, GroupId},
};

use crate::repository::group_mapper::{GroupInsertMapper, GroupMapper};
use crate::repository::stone_mapper::StoneMapper;
use crate::schema::{group_contacts, groups, stones};
use crate::{
    domain::grouped_stones::GroupedStones, repository::group_contacts_mapper::GroupContactsMapper,
};

pub struct GroupRepository {
    connection: SqliteConnection,
}

impl GroupRepository {
    pub fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    pub fn stone_is_in_group(&mut self, coord: Coord) -> bool {
        diesel::select(exists(stones::table.find(coord.into())))
            .get_result(&mut self.connection)
            .unwrap()
    }

    pub fn neighboring_groups(&mut self, coord: Coord) -> Vec<Group> {
        let x = coord.x();
        let y = coord.y();

        let groups_ids: Vec<i32> = stones::table
            .select(stones::group_id)
            .or_filter(stones::x.eq(x).and(stones::y.eq_any([y - 1, y + 1].iter())))
            .or_filter(stones::y.eq(y).and(stones::x.eq_any([y - 1, y + 1].iter())))
            .get_results(&mut self.connection)
            .unwrap();

        let groups: Vec<GroupMapper> = groups::table
            .filter(groups::id.eq_any(groups_ids.iter()))
            .get_results(&mut self.connection)
            .unwrap();

        let contacts: Vec<GroupContactsMapper> = group_contacts::table
            .or_filter(group_contacts::low_group_id.eq_any(groups_ids.iter()))
            .or_filter(group_contacts::high_group_id.eq_any(groups_ids.iter()))
            .get_results(&mut self.connection)
            .unwrap();

        let mut groups = groups.into_iter().map(|g_mapper| {
            let contacts = contacts
                .iter()
                .filter(|c| c.belongs_to(g_mapper.id()))
                .cloned()
                .collect();
            g_mapper.into_group(contacts)
        });

        groups_ids
            .into_iter()
            .map(|id| {
                groups
                    .find(|g| g.id().unwrap().into_primitive() == id)
                    .unwrap()
            })
            .collect()
    }

    pub fn merge_group(&mut self, target_group_id: GroupId, group: Group) {
        diesel::update(stones::table)
            .filter(stones::group_id.eq(group.id().unwrap().into_primitive()))
            .set(stones::group_id.eq(target_group_id.into_primitive()))
            .execute(&mut self.connection)
            .unwrap();
    }

    pub fn upsert_group(&mut self, group: &Group) -> GroupId {
        match group.id() {
            None => {
                let id = diesel::insert_into(groups::table)
                    .values::<GroupInsertMapper>(group.clone().into())
                    .get_result::<GroupMapper>(&mut self.connection)
                    .unwrap()
                    .id();
                GroupId::new(id)
            }

            Some(id) => {
                diesel::update(groups::table)
                    .set::<GroupMapper>(group.clone().into())
                    .execute(&mut self.connection)
                    .unwrap();
                id
            }
        }
    }

    pub fn delete_group(&mut self, group: Group) {
        let id = group.id().expect("Unassigned Group ID").into_primitive();
        diesel::delete(groups::table.find(id))
            .execute(&mut self.connection)
            .unwrap();
    }

    pub fn add_stone_to_group(&mut self, group_id: GroupId, coord: Coord) {
        diesel::insert_into(stones::table)
            .values(StoneMapper::new(coord, group_id))
            .execute(&mut self.connection)
            .unwrap();
    }

    pub fn get_groups_with_stones(&mut self) -> Vec<GroupedStones> {
        let stones: Vec<StoneMapper> = stones::table.get_results(&mut self.connection).unwrap();

        let stones_ids: HashSet<_> = stones
            .iter()
            .map(|stone: &StoneMapper| stone.group_id())
            .collect();

        let groups: Vec<GroupMapper> = groups::table
            .filter(groups::id.eq_any(stones_ids))
            .get_results(&mut self.connection)
            .unwrap();

        groups
            .into_iter()
            .map(|group| {
                let stones = stones
                    .iter()
                    .filter(|s| s.group_id() == group.id())
                    .collect();
                group.into_grouped_stones(stones)
            })
            .collect()
    }
}

impl Debug for GroupRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Group Repository")
    }
}
