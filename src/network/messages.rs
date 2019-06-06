use crate::player;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerBoundMessage {
    SetName(String),
    UpdateInputs(player::Inputs),
    Disconnect,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientBoundMessage {
    SetNameResponse { accepted: bool },
    PlayerUpdate(player::State),
}
