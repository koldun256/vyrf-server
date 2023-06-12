use std::sync::mpsc::{Receiver, Sender, channel};
use std::net::UdpSocket;
use std::thread;

pub enum ServerMsg {
    Val(i32)
}

pub enum ClientMsg {
    Increment
}

fn recv_msg(buf: &[u8; 4]) -> ClientMsg {
    ClientMsg::Increment
}

fn send_msg(msg: ServerMsg, socket: &UdpSocket, addr: String) {
    match msg {
        ServerMsg::Val(val) => socket.send_to(&val.to_be_bytes(), addr)
    };
}

pub fn connect() -> (Sender<(String, ServerMsg)>, Receiver<(String, ClientMsg)>) {
    let (tx1, rx1) = channel(); // from client to server
    let (tx2, rx2) = channel(); // from server to client
    let socket1 = UdpSocket::bind("127.0.0.1:8080").expect("cannot open udp socket");
    let socket2 = socket1.try_clone().unwrap();
    thread::spawn(move || while let Ok((addr, msg)) = rx2.recv() {
        send_msg(msg, &socket2, addr);
    });
    thread::spawn(move || {
        let mut answer = [0; 4];
        while let Ok((_, client)) = socket1.recv_from(&mut answer) {
            tx1.send((client.to_string(), recv_msg(&answer)));
        };
    });
    (tx2, rx1)
}