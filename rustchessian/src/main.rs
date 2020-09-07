use rustchessian::Board;

fn main(){
    let start = Board::BoardState::new();
    println!("{}", start);
    Board::BoardState::read(String::from("game"));
}
