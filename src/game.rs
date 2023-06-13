use vec2::Vec2;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use crate::udp_server::{ClientMsg, ServerMsg};
pub mod vec2;

pub enum GameObjectKind {
    Player { addr: String },
    Thing
}
struct GameObject {
    position: Vec2,
    id: u8,
    kind: GameObjectKind,
    movement: Vec2
}
impl GameObject {
    fn new_player(id: u8, addr: String) -> Self {
        GameObject { 
            position: (id as i16 * 100, id as i16 * 100).into(),
            id,
            kind: GameObjectKind::Player { addr },
            movement: (1, 1).into()
        }
    }
    fn init_msg(&self) -> ServerMsg {
        ServerMsg::AddObject {
            id: self.id, 
            kind: match &self.kind {
                GameObjectKind::Player { addr: _ } => 1,
                GameObjectKind::Thing => 2
            },
            pos: self.position
        }
    }
}
pub struct Game {
    last_id: u8,
    game_objects: Vec<GameObject>,
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
                thread::sleep(Duration::from_millis(50));
            }
        });
        tx
    }
    fn send_to_all_players(&self, msg: ServerMsg) {
        for game_object in &self.game_objects {
            if let GameObjectKind::Player { addr } = &game_object.kind {
                self.udp_tx.send((addr.clone(), msg)).expect("main thread died");
            }
        }
    }
    fn handle_msg(&mut self, addr: String, msg: ClientMsg) {
        match msg {
            ClientMsg::Register => {
                self.game_objects.push(GameObject::new_player(self.last_id + 1, addr.clone()));
                self.last_id += 1;

                let player_obj = self.game_objects.last().unwrap();
                for obj in &self.game_objects {
                    self.udp_tx.send((addr.clone(), obj.init_msg())).unwrap();
                }
                self.send_to_all_players(player_obj.init_msg());
                self.udp_tx.send((addr, ServerMsg::BindPlayer { id: player_obj.id })).unwrap();
            }
        }
    }
    fn get_by_id(&self, id: u8) -> Option<&GameObject> {
        for obj in &self.game_objects {
            if obj.id == id {
                return Some(obj)
            }
        }
        None
    }

    fn frame(&mut self) {
        let mut moved_ids: Vec<u8> = Vec::new();
        for obj in &mut self.game_objects {
            if obj.movement != Vec2::ZERO {
                obj.position += obj.movement;
                moved_ids.push(obj.id);
            }
        }
        for moved_id in moved_ids {
            self.send_to_all_players(ServerMsg::SetPosition {
                id: moved_id,
                pos: self.get_by_id(moved_id).unwrap().position 
            });
        }
    }
}