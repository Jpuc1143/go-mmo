use std::{collections::HashMap, sync::Mutex};

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, stream::SplitSink};
use tokio::sync::RwLock;

use crate::controller::game_server_message::GameServerMessage;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ClientId(u32);

#[derive(Debug)]
pub struct ClientManager {
    next_client_id: Mutex<ClientId>,
    message_senders: RwLock<HashMap<ClientId, SplitSink<WebSocket, Message>>>,
}

impl Default for ClientManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            next_client_id: Mutex::new(ClientId(0)),
            message_senders: RwLock::new(HashMap::new()),
        }
    }

    pub async fn register_client(&self, message_sender: SplitSink<WebSocket, Message>) -> ClientId {
        let id = self.next_client_id();
        self.message_senders
            .write()
            .await
            .insert(id, message_sender);
        id
    }

    pub async fn remove_client(&self, id: ClientId) {
        self.message_senders.write().await.remove(&id);
    }

    fn next_client_id(&self) -> ClientId {
        let mut id_guard = self.next_client_id.lock().unwrap();
        let id = *id_guard;
        id_guard.0 += 1;
        id
    }

    pub async fn send_message(&self, id: ClientId, msg: GameServerMessage) {
        if let Some(sender) = self.message_senders.write().await.get_mut(&id) {
            let msg = Self::serialize_message(msg);
            sender.send(msg).await.expect("Error sending message");
        }
    }

    fn serialize_message(msg: GameServerMessage) -> Message {
        let msg = serde_json::to_string(&msg).expect("Error serializing server message");
        Message::text(msg)
    }
}
