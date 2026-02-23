use diesel::prelude::*;

use super::super::domain::group::GroupId;

#[derive(Clone, Queryable, Selectable, Identifiable)]
#[diesel(primary_key(low_group_id, high_group_id))]
#[diesel(table_name = crate::schema::group_contacts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GroupContactsMapper {
    low_group_id: i32,
    high_group_id: i32,
    count: i32,
}

impl GroupContactsMapper {
    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn other_group_of(&self, id: GroupId) -> GroupId {
        if id.into_primitive() == self.low_group_id {
            return GroupId::new(self.high_group_id);
        } else {
            return GroupId::new(self.low_group_id);
        }
    }

    pub fn belongs_to(&self, group_id: i32) -> bool {
        return group_id == self.low_group_id || group_id == self.high_group_id;
    }
}
