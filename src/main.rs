use std::net::{UdpSocket, SocketAddr};
fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    let mut buf = [0; 10];
    let mut count: i32 = 0;

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let msg = &buf[..amt];
        count = count + msg[0] as i32;

        socket.send_to(&count.to_be_bytes(), &src)?;
    }
}