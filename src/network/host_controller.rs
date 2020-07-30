use crate::network::messages::*;
use crate::player::Player;
use crate::{Map, MapController, PlayerController, ShotController};
use bincode;
use crossbeam_channel::Sender;
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use piston::input::GenericEvent;
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ServerBound {
    pub message: ServerBoundMessage,
    pub player_name: Option<String>,
    pub source: SocketAddr,
}

pub struct HostController {
    players: Arc<Mutex<HashMap<SocketAddr, String>>>,
    unprocessed_inputs: Arc<Mutex<Vec<ServerBound>>>,
    tx: Sender<Packet>,
}

impl HostController {
    pub fn listen(addr: impl ToSocketAddrs) -> Result<Self, ErrorKind> {
        let mut socket = Socket::bind_with_config(addr, laminar::Config {
            heartbeat_interval: Some(std::time::Duration::from_secs(3)),
            ..laminar::Config::default()
        })?;
        let unprocessed_inputs = Arc::new(Mutex::new(vec![]));
        let players = Arc::new(Mutex::new(HashMap::new()));

        {
            let players = Arc::clone(&players);
            let unprocessed_inputs = Arc::clone(&unprocessed_inputs);
            let rx = socket.get_event_receiver();
            thread::spawn(move || loop {
                match rx.recv() {
                    Ok(SocketEvent::Packet(packet)) => {
                        let player_name = players.lock().unwrap().get(&packet.addr()).cloned();
                        let msg = bincode::deserialize(packet.payload()).unwrap();
                        match msg {
                            ServerBoundMessage::Connect => continue,
                            ServerBoundMessage::Disconnect => continue,
                            _ => (),
                        }
                        println!("decoded message {:?}", msg);
                        unprocessed_inputs.lock().unwrap().push(ServerBound {
                            message: msg,
                            player_name,
                            source: packet.addr(),
                        });
                    }
                    Ok(SocketEvent::Connect(addr)) => {
                        println!("{} connected", addr);
                        unprocessed_inputs.lock().unwrap().push(ServerBound {
                            message: ServerBoundMessage::Connect,
                            player_name: None,
                            source: addr,
                        });
                    }
                    _ => {}
                }
            });
        }

        let tx = socket.get_packet_sender();
        thread::spawn(move || socket.start_polling());

        Ok(Self {
            players,
            unprocessed_inputs,
            tx,
        })
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        e: &E,
        player_controller: &mut PlayerController,
        shot_controller: &mut ShotController,
        map_controller: &mut MapController,
    ) {
        if e.update_args().is_some() {
            self.update_game_state(player_controller, shot_controller, map_controller);

            let Self {
                unprocessed_inputs,
                players,
                tx,
                ..
            } = self;

            let mut unprocessed_inputs = unprocessed_inputs.lock().unwrap();
            let mut players = players.lock().unwrap();

            for player in player_controller.players.values_mut().filter(|p| p.dirty) {
                player.dirty = false;
                let msg =
                    ClientBoundMessage::PlayerUpdate(player.state.clone(), player.inputs.clone());
                for socket in players.keys() {
                    let packet = Packet::unreliable(*socket, bincode::serialize(&msg).unwrap());
                    tx.send(packet).unwrap();
                }
            }

            unprocessed_inputs.drain(..).for_each(|packet| {
                Self::process(packet, &mut players, player_controller, map_controller, tx)
            });

            for shot in shot_controller.shots.values_mut().filter(|shot| shot.dirty) {
                shot.dirty = false;
                let msg = ClientBoundMessage::ShotUpdate(shot.state.clone());
                for socket in players.keys() {
                    let packet = Packet::unreliable(*socket, bincode::serialize(&msg).unwrap());
                    tx.send(packet).unwrap();
                }
            }
        }
    }

    fn update_game_state(
        &self,
        player_controller: &mut PlayerController,
        shot_controller: &mut ShotController,
        map_controller: &mut MapController,
    ) {
        let players_alive = player_controller
            .players
            .values()
            .filter(|player| player.state.lives > 0)
            .count();
        let player_count = player_controller.players.len();

        if player_count > 1 && players_alive <= 1 {
            map_controller.map = Map::new();
            let Self { players, tx, .. } = self;
            let map = ClientBoundMessage::SetMap(map_controller.map.clone());
            Self::broadcast_reliable(tx, &players.lock().unwrap(), &map);

            player_controller.players.values_mut().for_each(|player| {
                player.state.lives = 20;
                player.dirty = true;
            });
            for shot in shot_controller.shots.values_mut() {
                shot.state.lives = 0;
                shot.dirty = true;
            }
        }
    }

    fn send_reliable(tx: &Sender<Packet>, target: &SocketAddr, msg: &ClientBoundMessage) {
        let packet = Packet::reliable_unordered(*target, bincode::serialize(msg).unwrap());
        tx.send(packet).unwrap();
    }

    fn broadcast_reliable(
        tx: &Sender<Packet>,
        players: &HashMap<SocketAddr, String>,
        msg: &ClientBoundMessage,
    ) {
        let data = bincode::serialize(msg).unwrap();
        for socket in players.keys() {
            let packet = Packet::reliable_unordered(*socket, data.clone());
            tx.send(packet).unwrap();
        }
    }

    fn process(
        packet: ServerBound,
        players: &mut HashMap<SocketAddr, String>,
        player_controller: &mut PlayerController,
        map_controller: &MapController,
        tx: &mut Sender<Packet>,
    ) {
        let player = Self::get_player(packet.player_name, player_controller);
        match packet.message {
            ServerBoundMessage::SetName(name) => {
                let accepted = Self::set_name(&name, packet.source, player_controller, tx);
                if accepted {
                    if let Some(color) = player_controller.get_free_color() {
                        players.insert(packet.source, name.clone());
                        let player = Player::new(name.clone(), 100.0, 100.0, color);
                        player_controller.players.insert(name.clone(), player);
                        let player = player_controller.players.get(&name).unwrap();

                        let new_player = ClientBoundMessage::PlayerUpdate(
                            player.state.clone(),
                            player.inputs.clone(),
                        );
                        Self::broadcast_reliable(tx, players, &new_player);
                    }
                }
            }
            ServerBoundMessage::UpdateInputs(inputs) => {
                if let Some(player) = player {
                    player.inputs = inputs;
                    player.dirty = true;
                }
            }
            ServerBoundMessage::Connect => {
                let map = ClientBoundMessage::SetMap(map_controller.map.clone());
                Self::send_reliable(tx, &packet.source, &map);

                for player in player_controller.players.values() {
                    let msg = ClientBoundMessage::PlayerUpdate(
                        player.state.clone(),
                        player.inputs.clone(),
                    );
                    Self::send_reliable(tx, &packet.source, &msg);
                }
            }
            ServerBoundMessage::Disconnect => {}
        }
    }

    fn set_name(
        name: &str,
        source: SocketAddr,
        player_controller: &mut PlayerController,
        tx: &mut Sender<Packet>,
    ) -> bool {
        let accepted = !player_controller
            .players
            .keys()
            .any(|exisiting_name| name == exisiting_name);

        let response = ClientBoundMessage::SetNameResponse { accepted };
        Self::send_reliable(tx, &source, &response);

        accepted
    }

    fn get_player(
        player_name: Option<String>,
        player_controller: &mut PlayerController,
    ) -> Option<&mut Player> {
        player_name.and_then(move |name| player_controller.players.get_mut(&name))
    }
}
