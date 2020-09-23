use rustchessian;
use std::io::{self, BufRead};

fn main() {

    let mut game = rustchessian::Game::new();

    loop {
        if game.get_gamestate() != rustchessian::Gamestate::InProgress {
            println!("GAME OVER!");
            break;
        }
        println!("{}", game);
        loop {
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
