use game::Game;
mod udp_server;
mod game;
fn main() {
    let (udp_sender, udp_reciever) = udp_server::launch();
    let game_sender = Game::launch(udp_sender.clone());
    while let Ok((address, message)) = udp_reciever.recv() {
        game_sender.send((address, message)).expect("game crashed");
    }
}
