use crate::{map, player, shot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerBoundMessage {
    SetName(String),
    UpdateInputs(player::Inputs),
    Disconnect,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientBoundMessage {
    SetNameResponse { accepted: bool },
    SetMap(map::Map),
    PlayerUpdate(player::State, player::Inputs),
    ShotUpdate(Vec<shot::Shot>),
}
