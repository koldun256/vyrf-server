use crate::game::{vector2::Vector2, GameEvent};
use crate::game::Game;
use crate::udp_server::ServerMessage;

use super::{GameObject, GameObjectKind, GameObjectKindMut};

pub struct Thing {
    id: u8,
    position: Vector2,
}

impl GameObject for Thing {
    fn get_id(&self) -> u8 { self.id }
    fn update(&mut self, _events: &Vec<GameEvent>) -> Box<dyn FnOnce(&mut Game) -> ()> { Box::new(|_| {}) }
    fn get_position(&self) -> Vector2 { self.position }
    fn generate_init_message(&self) -> ServerMessage {
        ServerMessage::AddObject {
            id: self.id,
            position: self.position,
            kind: 2
        }
    }
    fn downcast(&self) -> GameObjectKind {
        GameObjectKind::Thing(self)
    }
    fn downcast_mut(&mut self) -> GameObjectKindMut {
        GameObjectKindMut::Thing(self)
    }
}

impl Thing {
    pub fn new(id: u8, position: Vector2) -> Self {
        Thing { id, position }
    }
}
