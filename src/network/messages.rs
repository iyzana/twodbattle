use crate::player;
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use crate::player::State;

pub struct Input {
    pub message: InputMessage,
    pub player_name: Option<String>,
    pub source: SocketAddr,
}

#[derive(Deserialize)]
pub enum InputMessage {
    SetName(String),
    UpdateInputs(player::Inputs),
    Disconnect,
}

#[derive(Serialize)]
pub enum OutputMessage {
    NameRejected,
    NameAccepted,
    PlayerUpdate(State),
}
