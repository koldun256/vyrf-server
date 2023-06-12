use position::Position;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use crate::udp_server::ClientMsg;
mod position;

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

#[repr(u8)]
enum GameObjectKind {
    Player(Player) = 1
}

pub struct Game {
    last_id: u8,
    game_objects: Vec<GameObjectKind>,
    active: bool
}

impl Game {
    pub fn launch() -> mpsc::Sender<(String, ClientMsg)> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut game = Game {
                last_id: 0,
                game_objects: Vec::new(),
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
    fn handle_msg(&mut self, addr: String, msg: ClientMsg) {
        match msg {
            ClientMsg::Register => println!("registring client {}", addr)
        }
    }
    fn frame(&mut self) {}
}