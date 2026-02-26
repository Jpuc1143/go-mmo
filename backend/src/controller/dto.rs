use serde::{Deserialize, Serialize};

use crate::domain::{
    color::Color,
    group::{Group, GroupId},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupDto {
    id: GroupId,
    color: Color,
}

impl From<Group> for GroupDto {
    fn from(value: Group) -> Self {
        Self {
            id: value.id().unwrap(),
            color: value.color(),
        }
    }
}
