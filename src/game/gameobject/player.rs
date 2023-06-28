use std::sync::mpsc::Sender;

use crate::game::{vector2::Vector2, GameEvent};
use crate::game::Game;
use crate::udp_server::{ServerMessage, ClientMessage};

use super::{GameObject, GameObjectKind, GameObjectKindMut};

pub struct Player {
    id: u8,
    position: Vector2,
    address: String,
    movement: Vector2,
    speed: i16,
    udp_tx: Sender<(String, ServerMessage)>,
}

impl GameObject for Player {
    fn get_id(&self) -> u8 { self.id }
    fn update(&mut self, events: &Vec<GameEvent>) -> Box<dyn FnOnce(&mut Game) -> ()> {
        for event in events {
            let GameEvent::IncomingMessage(address, message) = event;
            if address == &self.address {
                self.process_message(*message);
            }
        }
        self.position += self.movement;
        Box::new(|_| {})
    }
    fn get_position(&self) -> Vector2 { self.position }
    fn generate_init_message(&self) -> ServerMessage {
        ServerMessage::AddObject {
            id: self.id,
            position: self.position,
            kind: 1
        }
    }
    fn downcast(&self) -> GameObjectKind {
        GameObjectKind::Player(self)
    }
    fn downcast_mut(&mut self) -> GameObjectKindMut {
        GameObjectKindMut::Player(self)
    }
}

impl Player {
    pub fn new(id: u8, address: String, game: &Game) -> Self {
        Player {
            id,
            position: Vector2::ZERO,
            address,
            movement: Vector2::ZERO,
            udp_tx: game.udp_sender.clone(),
            speed: 4
        }
    }
    pub fn send(&self, message: ServerMessage) {
        self.udp_tx.send((self.address.clone(), message)).expect("main thread died lol");
    }

    pub fn get_address(&self) -> &String { &self.address }
    fn process_message(&mut self, message: ClientMessage) {
        if let ClientMessage::SetDirection(direction) = message {
            self.movement = direction * self.speed;
        }
    }
}
