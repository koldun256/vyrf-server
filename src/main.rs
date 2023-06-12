use std::collections::HashSet;
use udp_server::{ClientMsg, ServerMsg};
mod udp_server;
fn main() -> std::io::Result<()> {
    let (udp_tx, udp_rx) = udp_server::connect();
    let mut clients: HashSet<String> = HashSet::new();

    while let Ok((client, msg)) = udp_rx.recv() {
        match msg {
            ClientMsg::Register => clients.insert(client)
        };
    }

    Ok(())
}