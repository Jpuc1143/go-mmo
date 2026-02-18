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

    pub fn place_stone(&self, coord: Coord, color: Color) -> Result<(), InvalidMoveError> {
        let (group_id, neighbors) = self.group_repository.assigned_group_and_neighbors(coord);
        if group_id.is_some() {
            return Err(InvalidMoveError);
        }

        let mut assigned_group = Group::merge_or_create(color, neighbors.iter());

        for group in neighbors.into_iter() {
            if group.id() == assigned_group.id() {
                continue;
            } else if group.color() == color {
                self.group_repository
                    .merge_group(assigned_group.id(), group);
            } else {
                if group.in_atari() {
                    assigned_group.remove_adjacent_group(group.id());
                    self.group_repository.delete_group(group);
                } else {
                    assigned_group.add_adjacent_group(group.id());
                }
            }
        }

        if assigned_group.is_dead() {
            return Err(InvalidMoveError);
        }

        self.group_repository
            .add_stone_to_group(assigned_group.id(), coord);
        self.group_repository.upsert_group(assigned_group);
        Ok(())
    }
}
