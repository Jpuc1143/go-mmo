use std::sync::Arc;

use tokio::sync::mpsc::Sender;

use crate::controller::{
    client_manager::{ClientId, ClientManager},
    game_client_command::GameClientCommand,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub client_manager: Arc<ClientManager>,
    pub command_sender: Sender<(ClientId, GameClientCommand)>,
}
