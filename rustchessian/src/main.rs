use rustchessian;

fn main() {
    let mut game = rustchessian::Game::new();
    println!("{}", game);
    let moves = game.gen_move_from_string("a2");
    game.make_move(moves[0]);
    println!("{}", game);
    println!("\n BOARD IS INVERTED, UPSIDE DOWN AND MIRRORED!!\n");
}
