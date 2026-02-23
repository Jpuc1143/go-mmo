use crate::domain::move_changes::MoveChanges;

use super::coord::Coord;

use super::group::Group;

use super::super::repository::group_repository::GroupRepository;

use super::color::Color;
use super::invalid_move_error::InvalidMoveError;

#[derive(Debug)]
pub struct GameService {
    group_repository: GroupRepository,
}

impl GameService {
    pub fn new(group_repository: GroupRepository) -> Self {
        Self { group_repository }
    }

    pub fn place_stone(
        &mut self,
        coord: Coord,
        color: Color,
    ) -> Result<MoveChanges, InvalidMoveError> {
        if self.group_repository.stone_is_in_group(coord) {
            return Err(InvalidMoveError);
        }

        let neighbors = self.group_repository.neighboring_groups(coord);
        let mut assigned_group = Group::merge_or_create(color, neighbors.iter());
        let mut merged_groups = Vec::new();

        for group in neighbors.into_iter() {
            if group.id() == assigned_group.id() {
                merged_groups.push(group.id().unwrap());
                continue;
            } else if group.color() == color {
                merged_groups.push(group.id().unwrap());
                self.group_repository
                    .merge_group(assigned_group.id().unwrap(), group);
            } else {
                if group.in_atari() {
                    assigned_group.remove_adjacent_group(group.id().unwrap());
                    self.group_repository.delete_group(group);
                } else {
                    assigned_group.add_adjacent_group(group.id().unwrap());
                }
            }
        }

        if assigned_group.is_dead() {
            return Err(InvalidMoveError);
        }

        let id = self.group_repository.upsert_group(assigned_group);
        self.group_repository.add_stone_to_group(id, coord);

        Ok(MoveChanges {
            coord,
            assigned_group: id,
            merged_groups,
        })
    }
}
