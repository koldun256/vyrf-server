// asdfsdaf
use std::sync::mpsc;
use std::time::Duration;
use std::{thread, time::Instant};

use crate::udp_server::{ClientMessage, ServerMessage};

use self::gameobject::player::Player;
use self::gameobject::{GameObject, GameObjectKind};
pub mod vector2;
pub mod gameobject;

pub enum GameEvent {
    IncomingMessage(String, ClientMessage)
}

pub struct Game {
    last_id: u8,
    game_objects: Vec<Box<dyn GameObject>>,
    events: Vec<GameEvent>,
    udp_sender: mpsc::Sender<(String, ServerMessage)>,
    udp_reciever: mpsc::Receiver<(String, ClientMessage)>,
    active: bool,
}

impl Game {
    pub fn launch(udp_sender: mpsc::Sender<(String, ServerMessage)>) -> mpsc::Sender<(String, ClientMessage)> {
        let (game_sender, game_reciever) = mpsc::channel();
        thread::spawn(move || {
            let mut game = Game {
                last_id: 0,
                game_objects: Vec::new(),
                udp_sender,
                udp_reciever: game_reciever,
                active: true,
                events: Vec::new(),
            };

            while game.active {
                let frame_start = Instant::now();
                game.update();
                thread::sleep(Duration::from_millis(20) - frame_start.elapsed()); // fixed 50 fps
            }
        });
        game_sender
    }

    fn send_to_all_players(&self, message: ServerMessage) {
        for game_object in &self.game_objects {
            if let GameObjectKind::Player(player) = game_object.downcast() {
                player.send(message);
            }
        }
    }

    fn create_id(&mut self) -> u8 {
        self.last_id += 1;
        self.last_id
    }

    fn add_object(&mut self, object: Box<dyn GameObject>) {
        self.send_to_all_players(object.generate_init_message());
        self.game_objects.push(object);
    }

    fn register_player(&mut self, address: String) {
        let new_player = Box::new(Player::new(
            self.create_id(),
            address,
            self
        ));
        for object in &self.game_objects {
            new_player.send(object.generate_init_message());
        }
        self.send_to_all_players(new_player.generate_init_message());
        new_player.send(new_player.generate_init_message());
        new_player.send(ServerMessage::BindPlayer { id: new_player.get_id() });
        self.add_object(new_player);
    }

    fn get_by_id(&self, id: u8) -> Option<&Box<dyn GameObject>> {
        for object in &self.game_objects {
            if object.get_id() == id {
                return Some(object);
            }
        }
        None
    }

    fn get_by_id_mut(&mut self, id: u8) -> Option<&mut Box<dyn GameObject>> {
        for object in &mut self.game_objects {
            if object.get_id() == id {
                return Some(object);
            }
        }
        None
    }

    fn update(&mut self) {
        self.events = Vec::new();
        while let Some((address, message)) = self.udp_reciever.try_iter().next() {
            if let ClientMessage::Register = message {
                self.register_player(address);
            } else {
                self.events.push(GameEvent::IncomingMessage(address, message));
            }
        }
        let mut dispatches: Vec<Box<dyn FnOnce(&mut Game) -> ()>> = Vec::new();
        for object in &mut self.game_objects {
            dispatches.push(object.update(&self.events));
        }
        for dispatch in dispatches {
            dispatch(self);
        }
        for object in &self.game_objects {
            self.send_to_all_players(ServerMessage::SetPosition {
                id: object.get_id(),
                position: object.get_position(),
            })
        }
    }
}
