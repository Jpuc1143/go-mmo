use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::{
    controller::{client_manager::ClientId, game_client_command::GameClientCommand},
    domain::{color::Color, coord::Coord, game_service::GameService},
};

use super::client_manager::ClientManager;

#[derive(Debug)]
pub struct GameController {
    game_service: GameService,
    client_manager: Arc<ClientManager>,
    command_sender: Sender<(ClientId, GameClientCommand)>,
    command_receiver: Receiver<(ClientId, GameClientCommand)>,
}

impl GameController {
    pub fn new(game_service: GameService, client_manager: Arc<ClientManager>) -> Self {
        let (command_sender, command_receiver) = channel(32);
        Self {
            game_service,
            client_manager,
            command_sender,
            command_receiver,
        }
    }

    pub fn command_sender(&self) -> Sender<(ClientId, GameClientCommand)> {
        self.command_sender.clone()
    }

    pub async fn start(mut self) {
        while let Some((id, msg)) = self.command_receiver.recv().await {
            match msg {
                GameClientCommand::PlaceStone { coord, color } => {
                    self.place_stone(id, coord, color).await
                }
            }
        }
    }

    pub async fn place_stone(&mut self, id: ClientId, coord: Coord, color: Color) {
        if let Ok(move_changes) = self.game_service.place_stone(coord, color) {
            let msg = move_changes.into();
            self.client_manager.send_message(id, msg).await;
        };
    }
}
