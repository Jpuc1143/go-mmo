use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use go_mmo::{
    domain::{color::Color, coord::Coord, game_service::GameService},
    repository::group_repository::GroupRepository,
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn get_game_service() -> GameService {
    let database_url = ":memory:";
    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    let repo = GroupRepository::new(connection);
    GameService::new(repo)
}

#[test]
fn place_stone() {
    let mut game_service = get_game_service();
    game_service
        .place_stone(Coord::new(0, 0), Color::Black)
        .unwrap();
}

#[test]
fn place_l_shape_group() {
    let mut game_service = get_game_service();

    game_service
        .place_stone(Coord::new(0, 1), Color::Black)
        .unwrap();
    game_service
        .place_stone(Coord::new(1, 0), Color::Black)
        .unwrap();

    let changes = game_service
        .place_stone(Coord::new(0, 0), Color::Black)
        .unwrap();

    assert_eq!(changes.merged_groups_ids.len(), 2)
}
