use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::{
    controller::{
        client_manager::ClientId, game_client_command::GameClientCommand,
        game_server_message::GameServerMessage,
    },
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
                GameClientCommand::RequestConfiguration => self.get_board(id).await,
                GameClientCommand::PlaceStone { coord, color } => {
                    self.place_stone(coord, color).await
                }
            }
        }
    }

    pub async fn get_board(&mut self, id: ClientId) {
        // TODO spawn blocking
        let msg = GameServerMessage::BoardData {
            grouped_stones: self
                .game_service
                .get_board()
                .into_iter()
                .map(|x| x.into())
                .collect(),
        };

        self.client_manager.send_message(id, msg).await;
    }

    pub async fn place_stone(&mut self, coord: Coord, color: Color) {
        // TODO spawn blocking
        if let Ok(move_changes) = self.game_service.place_stone(coord, color) {
            let msg = move_changes.into();
            self.client_manager.broadcast_message(msg).await;
        } else {
            println!("Invalid move {:?} {:?}", coord, color);
        };
    }
}
