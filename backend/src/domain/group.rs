use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use serde::{Deserialize, Serialize};

use super::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupId(i32);

impl GroupId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn into_primitive(self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    id: Option<GroupId>,
    color: Color,
    max_liberties: u32,
    adjacent_groups: HashMap<GroupId, u32>,
}

impl Group {
    pub fn new(
        id: GroupId,
        color: Color,
        max_liberties: u32,
        adjacent_groups: HashMap<GroupId, u32>,
    ) -> Self {
        Self {
            id: Some(id),
            color,
            max_liberties,
            adjacent_groups,
        }
    }

    pub fn merge_or_create<'a>(color: Color, groups: impl Iterator<Item = &'a Group>) -> Group {
        let groups: Vec<_> = groups.filter(|g| g.color() == color).collect();
        let assigned_group_id = groups.first().map(|g| g.id().unwrap());

        let mut assigned_group = Group {
            id: assigned_group_id,
            color,
            max_liberties: 4,
            adjacent_groups: HashMap::new(),
        };

        let mut already_visited = HashSet::new();
        for group in groups.iter() {
            if !already_visited.contains(&group.id()) {
                assigned_group.max_liberties += group.max_liberties;
                group.adjacent_groups.iter().for_each(|(id, count)| {
                    *assigned_group.adjacent_groups.entry(*id).or_insert(0) += count;
                });
                already_visited.insert(group.id());
            }

            assigned_group.max_liberties -= 2;
        }

        assigned_group
    }

    pub fn id(&self) -> Option<GroupId> {
        self.id
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn max_liberties(&self) -> u32 {
        self.max_liberties
    }

    pub fn liberties(&self) -> u32 {
        self.max_liberties - self.adjacent_groups.values().sum::<u32>()
    }

    pub fn in_atari(&self) -> bool {
        self.liberties() == 1
    }

    pub fn is_dead(&self) -> bool {
        self.liberties() == 0
    }

    pub fn add_adjacent_group(&mut self, id: GroupId) {
        let count = self.adjacent_groups.entry(id).or_insert(0);
        *count += 1
    }

    pub fn remove_adjacent_group(&mut self, id: GroupId) {
        self.adjacent_groups.remove(&id);
    }
}
