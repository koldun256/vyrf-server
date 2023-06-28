use std::sync::mpsc::{Receiver, Sender, channel};
use std::net::UdpSocket;
use std::thread;

use crate::game::vector2::Vector2;

enum Error {
    InvalidMessage
}
#[derive(Clone, Copy)]
pub enum ServerMessage { 
    AddObject { id: u8, kind: u8, position: Vector2 },
    BindPlayer { id: u8 },
    SetPosition { id: u8, position: Vector2 }
}

#[derive(Clone, Copy)]
pub enum ClientMessage {
    Register,
    SetDirection(Vector2)
}

fn parse_message(buf: &[u8; 10]) -> Result<ClientMessage, Error> {
    match buf[0] {
        0 => Ok(ClientMessage::Register),
        1 => Ok(ClientMessage::SetDirection(match buf[1] {
            0 => (-1, -1).into(),
            1 => (0, -1).into(),
            2 => (1, -1).into(),
            3 => (-1, 0).into(),
            4 => (0, 0).into(),
            5 => (1, 0).into(),
            6 => (-1, 1).into(),
            7 => (0, 1).into(),
            8 => (1, 1).into(),
            _ => return Err(Error::InvalidMessage)
        })),
        _ => Err(Error::InvalidMessage)
    }
}

fn gen_payload(message: ServerMessage) -> [u8; 10] { 
    let mut payload = [0; 10];
    match message {
        ServerMessage::AddObject { id, kind, position } => {
            payload[0] = 0;
            payload[1] = id;
            payload[2] = kind;
            let x_bytes = position.x.to_be_bytes();
            let y_bytes = position.y.to_be_bytes();
            payload[3] = x_bytes[0];
            payload[4] = x_bytes[1];
            payload[5] = y_bytes[0];
            payload[6] = y_bytes[1];
        },
        ServerMessage::BindPlayer { id } => {
            payload[0] = 1;
            payload[1] = id;
        },
        ServerMessage::SetPosition { id, position } => {
            payload[0] = 2;
            payload[1] = id;
            let x_bytes = position.x.to_be_bytes();
            let y_bytes = position.y.to_be_bytes();
            payload[2] = x_bytes[0];
            payload[3] = x_bytes[1];
            payload[4] = y_bytes[0];
            payload[5] = y_bytes[1];
        },
    }
    payload
}

pub fn launch() -> (Sender<(String, ServerMessage)>, Receiver<(String, ClientMessage)>) {
    let (udp_sender, main_receiver) = channel();
    let (main_sender, udp_receiver) = channel();
    let receiving_socket = UdpSocket::bind("127.0.0.1:8080").expect("cannot open udp socket");
    let sending_socket = receiving_socket.try_clone().unwrap();
    thread::spawn(move || {
        let mut payload = [0; 10];
        while let Ok((_, client)) = receiving_socket.recv_from(&mut payload) {
            match parse_message(&payload) {
                Ok(message) => udp_sender.send((client.to_string(), message)).unwrap(),
                Err(Error::InvalidMessage) => {
                    println!("invalid message {:?} from client", &payload);
                }
            }
        };
    });
    thread::spawn(move || while let Ok((address, message)) = udp_receiver.recv() {
        sending_socket.send_to(&gen_payload(message), address).unwrap();
    });
    (main_sender, main_receiver)
}
