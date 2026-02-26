use std::sync::Arc;

use diesel::prelude::*;

use axum::{Router, routing::get};

use go_mmo::{
    controller::{
        app_state::AppState, client_manager::ClientManager, game_controller::GameController,
        websocket_handler::websocket_handler,
    },
    domain::game_service::GameService,
    repository::group_repository::GroupRepository,
};

#[tokio::main]
async fn main() {
    // TODO env vars

    let database_url = ":memory:";
    let connection = SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let group_repository = GroupRepository::new(connection);
    let game_service = GameService::new(group_repository);

    let client_manager = Arc::new(ClientManager::new());
    let game_controller = GameController::new(game_service, Arc::clone(&client_manager));

    let state = AppState {
        client_manager,
        command_sender: game_controller.command_sender(),
    };

    tokio::spawn(game_controller.start());

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
