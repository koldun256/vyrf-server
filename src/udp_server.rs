use std::sync::mpsc::{Receiver, Sender, channel};
use std::net::UdpSocket;
use std::thread;

use crate::game::vec2::Vec2;

enum Error {
    InvalidMessage
}
#[derive(Clone, Copy)]
pub enum ServerMsg { 
    AddObject { id: u8, kind: u8, pos: Vec2 },
    BindPlayer { id: u8 },
    SetPosition { id: u8, pos: Vec2 }
}

pub enum ClientMsg {
    Register
}

fn parse_msg(buf: &[u8; 10]) -> Result<ClientMsg, Error> {
    if buf == &[0u8; 10] {
        Ok(ClientMsg::Register)
    } else {
        Err(Error::InvalidMessage)
    }
}

fn gen_payload(msg: ServerMsg) -> [u8; 10] { 
    let mut payload = [0; 10];
    match msg {
        ServerMsg::AddObject { id, kind, pos } => {
            payload[0] = 0;
            payload[1] = id;
            payload[2] = kind;
            let x_bytes = pos.x.to_be_bytes();
            let y_bytes = pos.y.to_be_bytes();
            payload[3] = x_bytes[0];
            payload[4] = x_bytes[1];
            payload[5] = y_bytes[0];
            payload[6] = y_bytes[1];
        },
        ServerMsg::BindPlayer { id } => {
            payload[0] = 1;
            payload[1] = id;
        },
        ServerMsg::SetPosition { id, pos } => {
            payload[0] = 2;
            payload[1] = id;
            let x_bytes = pos.x.to_be_bytes();
            let y_bytes = pos.y.to_be_bytes();
            payload[2] = x_bytes[0];
            payload[3] = x_bytes[1];
            payload[4] = y_bytes[0];
            payload[5] = y_bytes[1];
        },
    }
    payload
}

pub fn connect() -> (Sender<(String, ServerMsg)>, Receiver<(String, ClientMsg)>) {
    let (tx1, rx1) = channel(); // from client to server
    let (tx2, rx2) = channel(); // from server to client
    let socket1 = UdpSocket::bind("127.0.0.1:8080").expect("cannot open udp socket");
    let socket2 = socket1.try_clone().unwrap();
    thread::spawn(move || {
        let mut payload = [0; 10];
        while let Ok((_, client)) = socket1.recv_from(&mut payload) {
            match parse_msg(&payload) {
                Ok(msg) => tx1.send((client.to_string(), msg)).unwrap(),
                Err(Error::InvalidMessage) => {
                    println!("invalid msg {:?} from client", &payload);
                }
            }
        };
    });
    thread::spawn(move || while let Ok((addr, msg)) = rx2.recv() {
        socket2.send_to(&gen_payload(msg), addr).unwrap();
    });
    (tx2, rx1)
}