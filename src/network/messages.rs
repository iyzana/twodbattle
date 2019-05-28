use crate::player;
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use crate::player::State;

pub struct ServerBound {
    pub message: ServerBoundMessage,
    pub player_name: Option<String>,
    pub source: SocketAddr,
}

#[derive(Deserialize)]
pub enum ServerBoundMessage {
    SetName(String),
    UpdateInputs(player::Inputs),
    Disconnect,
}

#[derive(Serialize)]
pub enum ClientBoundMessage {
    NameRejected,
    NameAccepted,
    PlayerUpdate(State),
}
