use std::collections::HashSet;
use udp_server::{ClientMsg, ServerMsg};
mod udp_server;
fn main() -> std::io::Result<()> {
    let (udp_tx, udp_rx) = udp_server::connect();
    let mut count: i32 = 0;
    let mut clients: HashSet<String> = HashSet::new();

    while let Ok((client, msg)) = udp_rx.recv() {
        clients.insert(client);

        match msg {
            ClientMsg::Increment => count += 1
        }

        for client in &clients {
            udp_tx.send((client.to_string(), ServerMsg::Val(count)));
        }
    }

    Ok(())
}