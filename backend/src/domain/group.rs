use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use super::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GroupId(u64);

#[derive(Debug)]
pub struct Group {
    id: Option<GroupId>,
    color: Color,
    possible_liberties: u32,
    adjacent_groups: HashMap<GroupId, u32>,
}

impl Group {
    pub fn merge_or_create<'a>(color: Color, groups: impl Iterator<Item = &'a Group>) -> Group {
        let groups: Vec<_> = groups.filter(|g| g.color() == color).collect();
        let assigned_group_id = groups.first().map(|g| g.id());

        let mut assigned_group = Group {
            id: assigned_group_id,
            color,
            possible_liberties: 4,
            adjacent_groups: HashMap::new(),
        };

        let mut already_visited = HashSet::new();
        for group in groups.iter() {
            if already_visited.contains(&group.id()) {
                assigned_group.possible_liberties += group.possible_liberties;
                group.adjacent_groups.iter().for_each(|(id, count)| {
                    *assigned_group.adjacent_groups.entry(*id).or_insert(0) += count;
                });
                already_visited.insert(group.id());
            }

            assigned_group.possible_liberties -= 2;
        }

        assigned_group
    }

    pub fn id(&self) -> GroupId {
        self.id.expect("Group has no assigned ID yet")
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn liberties(&self) -> u32 {
        self.possible_liberties - self.adjacent_groups.values().fold(0, |a, b| a + b)
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
