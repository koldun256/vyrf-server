use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    let mut buf = [0; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let reply = &buf[..amt];

        for byte in reply {
            print!("{} ", byte);
        }
        println!();

        socket.send_to(&[0x01, 0x02, 0x03], &src)?;
    }
}