use axum::{
    extract::{
        State,
        ws::{WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use futures_util::stream::StreamExt;

use crate::controller::{app_state::AppState, game_client_command::GameClientCommand};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, app_state))
}

async fn handle_websocket(socket: WebSocket, app_state: AppState) {
    let AppState {
        client_manager,
        command_sender,
    } = app_state;
    let (message_sender, mut command_receiver) = socket.split();

    let id = client_manager.register_client(message_sender).await;

    let cmd = GameClientCommand::RequestConfiguration;
    command_sender.send((id, cmd)).await.unwrap();

    while let Some(cmd) = command_receiver.next().await {
        let cmd = cmd
            .expect("Error reading client command")
            .into_text()
            .unwrap();
        let cmd = serde_json::from_str(cmd.as_str()).expect("Unknown client command");
        command_sender.send((id, cmd)).await.ok();
    }

    client_manager.remove_client(id).await;
}
