use game::Game;
mod udp_server;
mod game;
fn main() {
    let (udp_tx, udp_rx) = udp_server::connect();
    let game_tx = Game::launch(udp_tx.clone());
    println!("test commit");
    while let Ok((addr, msg)) = udp_rx.recv() {
        game_tx.send((addr, msg)).expect("game crashed");
    }
}
