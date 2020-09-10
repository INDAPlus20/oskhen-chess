use rustchessian::Board;

fn main() {
    let start = Board::BoardState::new();
    println!("{}", start);
    let game = Board::BoardState::read(String::from("game"));
    println!("{}", game);
    game.move_from_string(String::from("0,1"));
}