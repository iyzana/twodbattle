use crate::network::messages::*;
use crate::player::Player;
use crate::LocalInputController;
use crate::MapController;
use crate::PlayerController;
use crate::ShotController;
use bincode;
use crossbeam::Sender;
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use piston::input::GenericEvent;
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Deserialize)]
struct ClientBound {
    message: ClientBoundMessage,
}

pub struct ClientController {
    host: SocketAddr,
    unprocessed_inputs: Arc<Mutex<Vec<ClientBound>>>,
    tx: Sender<Packet>,
}

impl ClientController {
    pub fn connect(host: SocketAddr, local: SocketAddr) -> Result<Self, ErrorKind> {
        let (mut socket, mut tx, rx) = Socket::bind(local)?;
        let unprocessed_inputs = Arc::new(Mutex::new(vec![]));

        {
            let unprocessed_inputs = Arc::clone(&unprocessed_inputs);
            thread::spawn(move || loop {
                match rx.recv() {
                    Ok(SocketEvent::Packet(packet)) => {
                        // ignore messages that are not from host
                        if packet.addr() != host {
                            break;
                        }

                        unprocessed_inputs.lock().unwrap().push(ClientBound {
                            message: bincode::deserialize(packet.payload()).unwrap(),
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

        Self::set_name(&host, String::from("client"), &mut tx);

        Ok(Self {
            host,
            unprocessed_inputs,
            tx,
        })
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        e: &E,
        player_controller: &mut PlayerController,
        map_controller: &mut MapController,
        shot_controller: &mut ShotController,
        local_input_controller: &mut Option<LocalInputController>,
    ) {
        if e.update_args().is_some() {
            let Self {
                host,
                unprocessed_inputs,
                tx,
                ..
            } = self;

            let mut unprocessed_inputs = unprocessed_inputs.lock().unwrap();

            unprocessed_inputs.drain(..).for_each(|packet| {
                Self::process(&host, packet, player_controller, shot_controller, map_controller, tx)
            });

            if let Some(local_input_controller) = local_input_controller.as_mut() {
                let player = player_controller
                    .players
                    .get_mut(&local_input_controller.local_player)
                    .expect("local player not present in player list");
                let msg = ServerBoundMessage::UpdateInputs(player.inputs.clone());
                player.inputs.jump = false;
                player.inputs.shoot = false;
                println!("sending msg {:?}", msg);
                let packet =
                    Packet::reliable_unordered(self.host, bincode::serialize(&msg).unwrap());
                tx.send(packet).unwrap();
            }
        }
    }

    fn process(
        host: &SocketAddr,
        packet: ClientBound,
        player_controller: &mut PlayerController,
        shot_controller: &mut ShotController,
        map_controller: &mut MapController,
        tx: &mut Sender<Packet>,
    ) {
        match packet.message {
            ClientBoundMessage::SetNameResponse { accepted } => {
                if !accepted {
                    Self::set_name(host, String::from("noname"), tx);
                }
            }
            ClientBoundMessage::SetMap(map) => {
                map_controller.map = map;
            }
            ClientBoundMessage::PlayerUpdate(state) => {
                // todo create new player if not found
                if let Some(player) = player_controller.players.get_mut(&state.name) {
                    println!("overriding player state: {:?}", state);
                    player.state = state;
                } else {
                    let mut player =
                        Player::new(state.name.clone(), 50.0, 50.0, [0.0, 1.0, 0.0, 1.0]);
                    player.state = state;
                    player_controller
                        .players
                        .insert(player.state.name.clone(), player);
                }
            }
            ClientBoundMessage::ShotUpdate(shots) => {
                shot_controller.shots = shots;
            }
        }
    }

    fn set_name(host: &SocketAddr, name: String, tx: &mut Sender<Packet>) {
        let msg = ServerBoundMessage::SetName(name);
        let packet = Packet::reliable_unordered(*host, bincode::serialize(&msg).unwrap());
        tx.send(packet).unwrap();
    }
}
