use rustchessian::Board;

fn main() {
    let start = Board::BoardState::new();
    println!("{}", start);
    let game = Board::BoardState::read(String::from("game"));
    println!("{}", game);
}
