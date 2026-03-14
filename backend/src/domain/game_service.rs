use crate::domain::grouped_stones::GroupedStones;
use crate::domain::stone_placed_changes::StonePlacedChanges;

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
    ) -> Result<StonePlacedChanges, InvalidMoveError> {
        if self.group_repository.stone_is_in_group(coord) {
            return Err(InvalidMoveError);
        }

        let neighbors = self.group_repository.neighboring_groups(coord);
        let mut assigned_group = Group::merge_or_create(color, neighbors.iter());
        let mut captured_groups_ids = Vec::new();
        let mut merged_groups_ids = Vec::new();

        for group in neighbors.into_iter() {
            if group.id() == assigned_group.id() {
                merged_groups_ids.push(group.id().unwrap());
                continue;
            } else if group.color() == color {
                merged_groups_ids.push(group.id().unwrap());
                self.group_repository
                    .merge_group(assigned_group.id().unwrap(), group);
            } else if group.in_atari() {
                captured_groups_ids.push(group.id().unwrap());
                assigned_group.remove_adjacent_group(group.id().unwrap());
                self.group_repository.delete_group(group);
            } else {
                assigned_group.add_adjacent_group(group.id().unwrap());
            }
        }

        if assigned_group.is_dead() {
            println!("{assigned_group:?}");
            return Err(InvalidMoveError);
        }

        let id = self.group_repository.upsert_group(&assigned_group);
        self.group_repository.add_stone_to_group(id, coord);
        assigned_group.set_id(id);

        Ok(StonePlacedChanges {
            coord,
            assigned_group,
            captured_groups_ids,
            merged_groups_ids,
        })
    }

    pub fn get_board(&mut self) -> Vec<GroupedStones> {
        self.group_repository.get_groups_with_stones()
    }
}
