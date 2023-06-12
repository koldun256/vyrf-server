use position::Position;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use crate::udp_server::{ClientMsg, ServerMsg};
pub mod position;

trait GameObject {
    fn get_position(&self) -> &Position;
    fn get_id(&self) -> &u8;
}

struct Player {
    position: Position,
    id: u8,
    addr: String
}

impl GameObject for Player {
    fn get_id(&self) -> &u8 { &self.id }
    fn get_position(&self) -> &Position { &self.position }
}

enum GameObjectKind {
    Player(Player)
}

pub struct Game {
    last_id: u8,
    game_objects: Vec<GameObjectKind>,
    udp_tx: mpsc::Sender<(String, ServerMsg)>,
    active: bool
}

impl Game {
    pub fn launch(udp_tx: mpsc::Sender<(String, ServerMsg)>) -> mpsc::Sender<(String, ClientMsg)> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut game = Game {
                last_id: 0,
                game_objects: Vec::new(),
                udp_tx,
                active: true
            };

            while game.active {
                while let Some((addr, msg)) = rx.try_iter().next() {
                    game.handle_msg(addr, msg);
                }
                game.frame();
                thread::sleep(Duration::from_millis(100));
            }
        });
        tx
    }
    fn send_to_all_players(&self, msg: ServerMsg) {
        for game_object in &self.game_objects {
            if let GameObjectKind::Player(player) = game_object {
                self.udp_tx.send((player.addr.clone(), msg)).expect("main thread died");
            }
        }
    }
    fn handle_msg(&mut self, addr: String, msg: ClientMsg) {
        match msg {
            ClientMsg::Register => {
                let id = self.last_id + 1;
                self.last_id += 1;
                let position: Position = (id as u16 * 100, id as u16 * 100).into();
                self.send_to_all_players(ServerMsg::AddObject {
                    id,
                    kind: 1,
                    x: position.x,
                    y: position.y
                });
                self.game_objects.push(GameObjectKind::Player(Player {
                    id,
                    position,
                    addr: addr.clone()
                }));
                for obj in &self.game_objects {
                    match obj {
                        GameObjectKind::Player(player) => {
                            self.udp_tx.send((addr.clone(), ServerMsg::AddObject {
                                id: player.id,
                                kind: 1,
                                x: player.position.x,
                                y: player.position.y
                            })).unwrap();
                        }
                    }
                }
                self.udp_tx.send((addr, ServerMsg::BindPlayer { id })).unwrap();
            }
        }
    }

    fn frame(&mut self) {}
}