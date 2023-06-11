use std::{net::{UdpSocket, SocketAddr}, collections::HashSet};
fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    let mut buf = [0; 10];
    let mut count: i32 = 0;
    let mut clients: HashSet<String> = HashSet::new();

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let msg = &buf[..amt];
        clients.insert(src.to_string());

        count = count + msg[0] as i32;

        for client in &clients {
            socket.send_to(&count.to_be_bytes(), client).expect("cannot send msg");
        }
    }
}