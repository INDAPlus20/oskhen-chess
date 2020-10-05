use rustchessian;
use std::io::{self, BufRead};

fn main() {
/*
    let content = "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 {This opening is called the Ruy Lopez.}
    4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
    11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
    Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
    23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
    hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
    35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
    Nf2 42. g4 Bd3 43. Re6 1/2-1/2";

    //rustchessian::pgn::parse_png(content.to_string());
    //return;
    //let mut game = rustchessian::Game::new();

    let gamestate ="
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

*/

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
