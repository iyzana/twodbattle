use crate::player;
use crate::player::Player;
use crate::PlayerController;
use bincode;
use laminar::{ErrorKind, Socket, SocketEvent};
use piston::input::GenericEvent;
use serde::Deserialize;
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;

struct InputPacket {
    source: SocketAddr,
    input_message: InputMessage,
}

#[derive(Deserialize)]
enum InputMessage {
    SetName(String),
    UpdateInputs(player::Inputs),
    Disconnect,
}

pub struct NetworkHostController {
    players: HashMap<SocketAddr, String>,
    unprocessed_inputs: Arc<Mutex<Vec<InputPacket>>>,
}

impl NetworkHostController {
    pub fn listen(addr: impl ToSocketAddrs) -> Result<Self, ErrorKind> {
        let (mut socket, tx, rx) = Socket::bind(addr)?;
        let unprocessed_inputs = Arc::new(Mutex::new(vec![]));

        {
            let unprocessed_inputs = Arc::clone(&unprocessed_inputs);
            thread::spawn(move || loop {
                match rx.recv() {
                    Ok(SocketEvent::Packet(packet)) => {
                        unprocessed_inputs.lock().unwrap().push(InputPacket {
                            source: packet.addr(),
                            input_message: bincode::deserialize(packet.payload()).unwrap(),
                        });
                    }
                    Ok(SocketEvent::Connect(addr)) => {}
                    _ => {}
                }
            });
        }

        thread::spawn(move || {
            socket.start_polling().unwrap();
        });

        let players = HashMap::new();
        Ok(Self {
            players,
            unprocessed_inputs,
        })
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E, player_controller: &mut PlayerController) {
        if e.update_args().is_some() {
            let Self {
                unprocessed_inputs,
                players,
                ..
            } = self;

            unprocessed_inputs
                .lock()
                .unwrap()
                .drain(..)
                .for_each(|packet| Self::process(players, packet, player_controller));
        }
    }

    fn process(
        players: &mut HashMap<SocketAddr, String>,
        packet: InputPacket,
        player_controller: &mut PlayerController,
    ) {
        match packet.input_message {
            InputMessage::SetName(name) => {
                if players
                    .values()
                    .any(|exisiting_name| name == *exisiting_name)
                {
                    // nope
                } else {
                    players.insert(packet.source, name.clone());
                    player_controller.players.insert(
                        name.clone(),
                        Player::new(name, 100.0, 100.0, [0.0, 1.0, 1.0, 1.0]),
                    );
                }
            }
            InputMessage::UpdateInputs(inputs) => {
                if let Some(player) = Self::get_player(players, packet.source, player_controller) {
                    player.inputs = inputs;
                }
            }
            InputMessage::Disconnect => {}
        }
    }

    fn get_player<'a>(
        players: &mut HashMap<SocketAddr, String>,
        addr: SocketAddr,
        player_controller: &'a mut PlayerController,
    ) -> Option<&'a mut Player> {
        let name = players.get(&addr)?;
        player_controller.players.get_mut(name)
    }
}
