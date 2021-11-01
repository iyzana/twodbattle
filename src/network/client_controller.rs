use crate::network::messages::*;
use crate::player::Player;
use crate::shot::Shot;
use crate::LocalInputController;
use crate::MapController;
use crate::PlayerController;
use crate::ShotController;
use anyhow::anyhow;
use crossbeam_channel::Sender;
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
    pub fn connect(
        host: SocketAddr,
        local: SocketAddr,
        name: Option<&str>,
    ) -> Result<Self, ErrorKind> {
        let mut socket = Socket::bind_with_config(
            local,
            laminar::Config {
                heartbeat_interval: Some(std::time::Duration::from_secs(3)),
                ..laminar::Config::default()
            },
        )?;
        let unprocessed_inputs = Arc::new(Mutex::new(vec![]));
        let rx = socket.get_event_receiver();

        {
            let unprocessed_inputs = Arc::clone(&unprocessed_inputs);
            thread::spawn(move || loop {
                match rx.recv() {
                    Ok(SocketEvent::Packet(packet)) => {
                        // ignore messages that are not from host
                        if packet.addr() != host {
                            break;
                        }

                        let message = bincode::deserialize(packet.payload()).unwrap();
                        println!("decoded message {:?}", message);
                        unprocessed_inputs
                            .lock()
                            .unwrap()
                            .push(ClientBound { message });
                    }
                    Ok(SocketEvent::Connect(_addr)) => {}
                    _ => {}
                }
            });
        }

        let mut tx = socket.get_packet_sender();
        if let Some(name) = name {
            Self::set_name(&host, name.to_string(), &mut tx);
        }

        thread::spawn(move || socket.start_polling());

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
    ) -> Result<(), anyhow::Error> {
        if e.update_args().is_some() {
            let Self {
                unprocessed_inputs,
                tx,
                ..
            } = self;

            let mut unprocessed_inputs = unprocessed_inputs.lock().unwrap();

            unprocessed_inputs.drain(..).try_for_each(|packet| {
                Self::process(
                    packet,
                    player_controller,
                    local_input_controller,
                    shot_controller,
                    map_controller,
                )
            })?;

            let player = local_input_controller
                .as_mut()
                .map(|l| &l.local_player)
                .and_then(|name| player_controller.players.get_mut(name));
            if let Some(player) = player {
                if player.dirty {
                    player.dirty = false;
                    let msg = ServerBoundMessage::UpdateInputs(player.inputs.clone());
                    println!("sending msg {:?}", msg);
                    let packet = Packet::unreliable(self.host, bincode::serialize(&msg).unwrap());
                    tx.send(packet).unwrap();
                }
            }
        }

        Ok(())
    }

    fn process(
        packet: ClientBound,
        player_controller: &mut PlayerController,
        local_input_controller: &Option<LocalInputController>,
        shot_controller: &mut ShotController,
        map_controller: &mut MapController,
    ) -> Result<(), anyhow::Error> {
        match packet.message {
            ClientBoundMessage::SetNameResponse { accepted } => {
                if !accepted {
                    return Err(anyhow!("name already taken"));
                }
            }
            ClientBoundMessage::SetMap(map) => {
                map_controller.map = map;
            }
            ClientBoundMessage::PlayerUpdate(state, inputs) => {
                if let Some(player) = player_controller.players.get_mut(&state.name) {
                    println!("overriding player");
                    if local_input_controller
                        .as_ref()
                        .map(|l| player.state.name != l.local_player)
                        .unwrap_or(true)
                    {
                        // always sync other players
                        println!("  {:?}", inputs);
                        player.state = state;
                        player.inputs = inputs;
                    } else if !player.inputs.left
                        && !player.inputs.right
                        && !player.inputs.jump
                        && player.on_ground
                    {
                        // sync local player when standing still
                        player.state = state;
                    }
                } else {
                    println!("creating new player: {:?}", state.name);
                    let mut player =
                        Player::new(state.name.clone(), 50.0, 50.0, [0.0, 0.0, 0.0, 1.0]);
                    player.state = state;
                    player.inputs = inputs;
                    player_controller
                        .players
                        .insert(player.state.name.clone(), player);
                }
            }
            ClientBoundMessage::ShotUpdate(shot_state) => {
                shot_controller
                    .shots
                    .entry(shot_state.id.clone())
                    .and_modify(|shot| shot.state = shot_state.clone())
                    .or_insert_with(|| {
                        let color = player_controller
                            .players
                            .get(&shot_state.id.owner)
                            .map(|player| player.state.color)
                            .unwrap_or([1.0; 4]);
                        Shot::from_state(shot_state, color)
                    });
            }
        }

        Ok(())
    }

    fn set_name(host: &SocketAddr, name: String, tx: &mut Sender<Packet>) {
        let msg = ServerBoundMessage::SetName(name);
        let packet = Packet::reliable_unordered(*host, bincode::serialize(&msg).unwrap());
        tx.send(packet).unwrap();
    }
}
