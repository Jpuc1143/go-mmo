use diesel::prelude::*;

use super::super::domain::group::GroupId;

#[derive(Clone, Queryable, Selectable, Identifiable, Insertable)]
#[diesel(primary_key(low_group_id, high_group_id))]
#[diesel(table_name = crate::schema::group_contacts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GroupContactsMapper {
    low_group_id: i32,
    high_group_id: i32,
    count: i32,
}

impl GroupContactsMapper {
    pub fn new(id0: GroupId, id1: GroupId, count: u32) -> Self {
        let id0 = id0.into_primitive();
        let id1 = id1.into_primitive();

        let low_group_id = if id0 < id1 { id0 } else { id1 };
        let high_group_id = if id0 > id1 { id0 } else { id1 };

        Self {
            low_group_id,
            high_group_id,
            count: count.try_into().unwrap(),
        }
    }

    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn other_group_of(&self, id: GroupId) -> GroupId {
        if id.into_primitive() == self.low_group_id {
            GroupId::new(self.high_group_id)
        } else {
            GroupId::new(self.low_group_id)
        }
    }

    pub fn belongs_to(&self, group_id: i32) -> bool {
        group_id == self.low_group_id || group_id == self.high_group_id
    }
}
