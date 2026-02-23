use diesel::prelude::*;

use super::super::domain::coord::Coord;

use super::super::domain::group::GroupId;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(belongs_to(GroupMapper))]
#[diesel(table_name = crate::schema::stones)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StoneMapper {
    x: i64,
    y: i64,
    group_id: i32,
}

impl StoneMapper {
    pub fn new(coord: Coord, group_id: GroupId) -> Self {
        Self {
            x: coord.x(),
            y: coord.y(),
            group_id: group_id.into_primitive(),
        }
    }
}
