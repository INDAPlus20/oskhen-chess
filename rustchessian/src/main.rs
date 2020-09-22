use rustchessian;
use std::io::{self, BufRead};

fn main() {
    //let mut game = rustchessian::Game::new();
    let gamestate ="
    RB XX XX XX KB BB NB RB
    PB PB PB PB PB PB PB PB
    XX XX XX XX XX XX XX XX
    XX XX XX XX XX XX XX XX
    XX XX XX XX XX XX XX XX
    XX XX XX XX XX XX XX XX
    XX XX XX XX PW PW PW PW
    RW XX XX XX KW BW NW RW
    ";
    let mut game = rustchessian::Game::board_from_blocks(gamestate);
    'outer: loop {
        println!("{}", game);
        'inner: loop {
            println!("Generate moves from square: ");
            let input = io::stdin().lock().lines().next().unwrap().unwrap();

            let moves = match game.gen_move_from_string(&input) {
                Ok(i) => i,
                Err(i) => {
                    println!("{}", i);
                    continue;
                },
            };
        
            println!("Choose move index: ");
            let input_index = io::stdin()
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .parse::<usize>()
                .unwrap()
                - 1;
            game.make_move(moves[input_index]);
            print!("\x1B[2J\x1B[1;1H"); // Clears terminal screen
            break;
        }
    }
    /*
    let moves = game.gen_move_from_string("a2");
    game.make_move(moves[1]);
    println!("{}", game);
    let moves = game.gen_move_from_string("a1");
    */
}
