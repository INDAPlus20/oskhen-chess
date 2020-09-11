use rustchessian;

fn main() {
    let game = rustchessian::Game::new();
    println!("{}", game);
    let moves = game.move_from_string("a2");
    println!("{:?}", moves);
}
