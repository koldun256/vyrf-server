use crate::game::vector2::Vector2;
use crate::game::Game;
use crate::udp_server::ServerMessage;

use self::player::Player;
use self::thing::Thing;

use super::GameEvent;

pub mod player;
pub mod thing;

pub trait GameObject {
    fn get_id(&self) -> u8;
    fn update(&mut self, events: &Vec<GameEvent>) -> Box<dyn FnOnce(&mut Game) -> ()>;
    fn get_position(&self) -> Vector2;
    fn generate_init_message(&self) -> ServerMessage;
    fn downcast(&self) -> GameObjectKind;
    fn downcast_mut(&mut self) -> GameObjectKindMut;
}

pub enum GameObjectKind<'a> {
    Player(&'a Player),
    Thing(&'a Thing),
}

pub enum GameObjectKindMut<'a> {
    Player(&'a mut Player),
    Thing(&'a mut Thing),
}
