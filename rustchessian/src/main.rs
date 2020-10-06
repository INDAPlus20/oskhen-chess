use rustchessian;
use std::io::{self, BufRead};
use std::fs;

fn main() {
    let PATH = "../replays/AdamsLast.pgn";
    let content = fs::read_to_string(PATH).unwrap();
    println!("{}", content);
    rustchessian::pgn::parse_png(content);
    return;
    //let mut game = rustchessian::Game::new();

    let gamestate = "
    RB XX BB QB KB XX XX RB
    XX XX PB XX BB PB PB PB
    PB XX NB PB XX NB XX XX
    XX PB XX XX PB XX XX XX
    XX XX XX XX PW XX XX XX
    XX BW PW XX XX NW XX XX
    PW PW XX PW XX PW PW PW
    RW NW BW QW RW XX KW XX
    ";

    let mut game = rustchessian::Game::board_from_blocks(gamestate);

    let mut game = rustchessian::Game::new();

    game.start_round();

    loop {
        if game.start_round() != rustchessian::Gamestate::InProgress {
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
                }
            };

            println!("Choose move index: ");
            let input_index = match io::stdin()
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .parse::<usize>()
            {
                Ok(i) => i - 1,
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
