mod game;

fn main() {
    let game = game::Game::new(&"R-gama".to_string());
    game.run();
}
