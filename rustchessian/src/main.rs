use rustchessian;
use std::io::{self, BufRead};

fn main() {

    let gamestate  ="
    RB NB BB XX KB BB NB RB
    PB PB PB XX PB PB PB PB
    XX XX XX XX XX XX XX XX
    QB XX XX XX XX XX XX XX
    XX XX XX XX XX XX XX XX
    XX XX XX XX XX XX XX XX
    PW PW PW XX PW PW PW PW
    RW NW BW QW KW BW NW RW
    ";
    let mut game = rustchessian::Game::board_from_blocks(gamestate);

    //let mut game = rustchessian::Game::new();

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
            let input_index = match io::stdin()
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .parse::<usize>() {
                    Ok(i) => i-1,
                    Err(i) => {
                        println!("{}", i);
                        continue;
                    }
                };
            if input_index >= moves.len() {
                println!("Invalid option!");    
                continue;
            }
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
