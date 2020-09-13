use rustchessian;
use std::io::{self, BufRead};

fn main() {
    let mut game = rustchessian::Game::new();
    loop {
        println!("{}", game);
        println!("Generate moves from square: ");
        let input = io::stdin().lock().lines().next().unwrap().unwrap();
        let moves = game.gen_move_from_string(&input);
        println!("Choose move index: ");
        let input_index = io::stdin().lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap() - 1;
        game.make_move(moves[input_index]);
        print!("\x1B[2J\x1B[1;1H"); // Clears terminal screen
        
    }
    /*
    let moves = game.gen_move_from_string("a2");
    game.make_move(moves[1]);
    println!("{}", game);
    let moves = game.gen_move_from_string("a1");
    */
}
