use crate::network::messages::*;
use crate::player::Player;
use crate::{MapController, PlayerController, ShotController};
use bincode;
use crossbeam::Sender;
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use piston::input::{Button, ButtonArgs, ButtonState, GenericEvent, Key};
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
        let mut socket = Socket::bind(addr)?;
        let unprocessed_inputs = Arc::new(Mutex::new(vec![]));
        let players = Arc::new(Mutex::new(HashMap::new()));

        {
            let players = Arc::clone(&players);
            let unprocessed_inputs = Arc::clone(&unprocessed_inputs);
            let rx = socket.get_event_receiver();
            thread::spawn(move || loop {
                match rx.recv() {
                    Ok(SocketEvent::Packet(packet)) => {
                        println!("got packet");
                        let player_name = players.lock().unwrap().get(&packet.addr()).cloned();
                        let msg = bincode::deserialize(packet.payload()).unwrap();
                        println!("decoded message {:?}", msg);
                        unprocessed_inputs.lock().unwrap().push(ServerBound {
                            message: msg,
                            player_name,
                            source: packet.addr(),
                        });
                    }
                    Ok(SocketEvent::Connect(addr)) => {
                        println!("{} connected", addr);
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
        shot_controller: &ShotController,
        map_controller: &MapController,
    ) {
        if e.update_args().is_some() {
            let Self {
                unprocessed_inputs,
                players,
                tx,
                ..
            } = self;

            let mut unprocessed_inputs = unprocessed_inputs.lock().unwrap();
            let mut players = players.lock().unwrap();

            unprocessed_inputs.drain(..).for_each(|packet| {
                Self::process(packet, &mut players, player_controller, map_controller, tx)
            });

            for player in player_controller.players.values() {
                let msg = ClientBoundMessage::PlayerUpdate(player.state.clone());
                for socket in players.keys() {
                    let packet = Packet::unreliable(*socket, bincode::serialize(&msg).unwrap());
                    tx.send(packet).unwrap();
                }
            }

            let msg = ClientBoundMessage::ShotUpdate(shot_controller.shots.clone());
            for socket in players.keys() {
                let packet = Packet::unreliable(*socket, bincode::serialize(&msg).unwrap());
                tx.send(packet).unwrap();
            }
        }

        if let Some(ButtonArgs {
            button: Button::Keyboard(Key::R),
            state: ButtonState::Press,
            ..
        }) = e.button_args()
        {
            let Self { players, tx, .. } = self;
            let map = ClientBoundMessage::SetMap(map_controller.map.clone());
            Self::broadcast_reliable(tx, &players.lock().unwrap(), map);
        }
    }

    fn broadcast_reliable(
        tx: &Sender<Packet>,
        players: &HashMap<SocketAddr, String>,
        msg: ClientBoundMessage,
    ) {
        for socket in players.keys() {
            let packet = Packet::reliable_unordered(*socket, bincode::serialize(&msg).unwrap());
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
                Self::set_name(
                    name,
                    packet.source,
                    players,
                    player_controller,
                    map_controller,
                    tx,
                );
            }
            ServerBoundMessage::UpdateInputs(inputs) => {
                if let Some(player) = player {
                    player.inputs = inputs;
                }
            }
            ServerBoundMessage::Disconnect => {}
        }
    }

    fn set_name(
        name: String,
        source: SocketAddr,
        players: &mut HashMap<SocketAddr, String>,
        player_controller: &mut PlayerController,
        map_controller: &MapController,
        tx: &mut Sender<Packet>,
    ) {
        let accepted = !player_controller
            .players
            .keys()
            .any(|exisiting_name| name == *exisiting_name);

        if accepted {
            players.insert(source, name.clone());
            player_controller.players.insert(
                name.clone(),
                Player::new(name, 100.0, 100.0, [0.0, 1.0, 1.0, 1.0]),
            );
        }

        let response = ClientBoundMessage::SetNameResponse { accepted };
        let packet = Packet::reliable_unordered(source, bincode::serialize(&response).unwrap());
        tx.send(packet).unwrap();
        let map = ClientBoundMessage::SetMap(map_controller.map.clone());
        let map = Packet::reliable_unordered(source, bincode::serialize(&map).unwrap());
        tx.send(map).unwrap();
    }

    fn get_player(
        player_name: Option<String>,
        player_controller: &mut PlayerController,
    ) -> Option<&mut Player> {
        player_name.and_then(move |name| player_controller.players.get_mut(&name))
    }
}
