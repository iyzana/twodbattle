use crate::player;
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
pub enum ServerBoundMessage {
    SetName(String),
    UpdateInputs(player::Inputs),
    Disconnect,
}

#[derive(Serialize, Deserialize)]
pub enum ClientBoundMessage {
    SetNameResponse { accepted: bool },
    PlayerUpdate(player::State),
}
