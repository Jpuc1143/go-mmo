use diesel::prelude::*;

use super::super::domain::coord::Coord;

use super::super::domain::group::GroupId;

#[derive(Queryable, Selectable, Insertable, Identifiable)]
#[diesel(primary_key(x, y))]
#[diesel(belongs_to(GroupMapper))]
#[diesel(table_name = crate::schema::stones)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StoneMapper {
    pub x: i64,
    pub y: i64,
    pub group_id: i32,
}

impl StoneMapper {
    pub fn new(coord: Coord, group_id: GroupId) -> Self {
        Self {
            x: coord.x(),
            y: coord.y(),
            group_id: group_id.into_primitive(),
        }
    }

    pub fn group_id(&self) -> i32 {
        self.group_id
    }

    pub fn coord(&self) -> Coord {
        Coord::new(self.x, self.y)
    }
}
