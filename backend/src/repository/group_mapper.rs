use diesel::prelude::*;

use crate::{
    domain::{group::GroupId, grouped_stones::GroupedStones},
    repository::{group_contacts_mapper::GroupContactsMapper, stone_mapper::StoneMapper},
};

use super::super::domain::{color::Color, group::Group};

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GroupMapper {
    id: i32,
    is_black: bool,
    max_liberties: i32,
}

impl GroupMapper {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn color(&self) -> Color {
        if self.is_black {
            Color::Black
        } else {
            Color::White
        }
    }

    pub fn into_group(self, contacts: Vec<GroupContactsMapper>) -> Group {
        let group_id = GroupId::new(self.id);
        let adjacents = contacts
            .into_iter()
            .map(|c| (c.other_group_of(group_id), c.count().try_into().unwrap()))
            .collect();

        Group::new(
            group_id,
            self.color(),
            self.max_liberties.try_into().unwrap(),
            adjacents,
        )
    }

    pub fn into_grouped_stones(self, stones: Vec<&StoneMapper>) -> GroupedStones {
        GroupedStones::new(
            GroupId::new(self.id),
            self.color(),
            stones.into_iter().map(|s| s.coord()).collect(),
        )
    }
}

impl From<Group> for GroupMapper {
    fn from(value: Group) -> Self {
        Self {
            id: value.id().expect("Unassigned Group ID").into_primitive(),
            is_black: matches!(value.color(), Color::Black),
            max_liberties: value.max_liberties().try_into().unwrap(),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GroupInsertMapper {
    is_black: bool,
    max_liberties: i32,
}

impl From<Group> for GroupInsertMapper {
    fn from(value: Group) -> Self {
        Self {
            is_black: matches!(value.color(), Color::Black),
            max_liberties: value.max_liberties().try_into().unwrap(),
        }
    }
}
