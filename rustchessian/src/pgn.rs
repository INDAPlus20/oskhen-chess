use crate as rustchessian;

fn main() {

    let content = "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 {This opening is called the Ruy Lopez.}
    4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
    11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
    Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
    23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
    hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
    35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
    Nf2 42. g4 Bd3 43. Re6 1/2-1/2";

    //parse_png(content);
    

}

pub fn parse_png(pgnfile: String) -> Vec<rustchessian::Action> {

    let strmoves = parse_png_to_strmoves(pgnfile);
    let mut actions: Vec<rustchessian::Action> = Vec::new();
    let mut game = rustchessian::Game::new();

    for strmove in strmoves {
        game.start_round();
        println!("player: {:?}", game.player);
        println!("{}", game);
        let available_moves = game.moveset.clone();
        let index = strmove.rfind(char::is_numeric);
        if index.is_none() {
            match strmove.len() {
                5 => {
                    println!("Queenside castling");
                    for maybe_moves in available_moves.values() {
                        for action in maybe_moves.iter() {
                            if action.movetype == rustchessian::Actiontype::Castling {
                                if action.to.coordinate.0 == 2 {
                                    println!("{:?}\n", action);
                                    actions.push(*action);
                                    game.make_move(*action);
                                    break;
                                }
                            }
                        }
                    }
                }
                3 => {
                    println!("Kingside castling");
                    for maybe_moves in available_moves.values() {
                        for action in maybe_moves.iter() {
                            if action.movetype == rustchessian::Actiontype::Castling {
                                println!("\nx\n");
                                if action.to.coordinate.0 == 6 {
                                    println!("here");
                                    println!("{:?}\n", action);
                                    actions.push(*action);
                                    game.make_move(*action);
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => panic!("Invalid string"),
            }
            continue;
        }
        let index = index.unwrap();
        let strcoordinate: &str =  &strmove[(index-1)..(index+1)];
        let to_coordinates = rustchessian::coordinate_from_string(strcoordinate).expect("Invalid coordinates!");

        let rank = match strmove.find(char::is_uppercase) {
            Some(i ) => match strmove.chars().nth(i).unwrap() {
                'P' => rustchessian::Rank::Pawn,
                'R' => rustchessian::Rank::Rook,
                'B' => rustchessian::Rank::Bishop,
                'Q' => rustchessian::Rank::Queen,
                'N' => rustchessian::Rank::Knight,
                'K' => rustchessian::Rank::King,
                _ => panic!("Piece letter not valid"),
            }
            None => rustchessian::Rank::Pawn,
        };

        let mut possible_actions: Vec<rustchessian::Action> = Vec::new();

        for maybe_moves in available_moves.values() {
            for action in maybe_moves.iter() {
                if action.to.coordinate == ((to_coordinates.0 as isize), (to_coordinates.1 as isize)) {
                    //println!("maybe? {:?}. but no bcs {:?} isnt {:?}", action, action.from.piece.unwrap().rank, rank);
                    if action.from.piece.unwrap().rank == rank {
                        possible_actions.push(*action);
                    }
                    
                }
            }
        }

        if possible_actions.len() == 1 {
            let this_action = possible_actions[0];
            println!("{:?}\n", this_action);
            actions.push(this_action);
            game.make_move(this_action);
            continue;
        }

        println!("Broke on: {:?}\n\n with rank {:?}", strmove, rank);
        break;
    }

    actions
}

fn parse_png_to_strmoves(pgnfile: String) -> Vec<String> {
    
    let mut content = String::new();

    let mut comment_flag: bool = false;
    for c in pgnfile.chars() {
        if c == '{' {
            comment_flag = true;
        }
        if comment_flag {
            if c == '}' {
                comment_flag = false;
            }
            continue;
        }
        content.push(c);
    }

    let mut full_turns: Vec<String> = content
    .split(|c| c == '.')
    .map(|s| {
        s.trim()
            .split_whitespace()
            .enumerate()
            .filter(|&(i, _)| i < 2)
            .map(|(_, e)| e)
            .collect::<Vec<&str>>()
            .join(" ")
    }).collect::<Vec<String>>();
    
    full_turns.remove(0);
    let mut half_turns:Vec<String>=full_turns.iter().flat_map(|s|s.split_whitespace()).map(|s|s.to_string()).collect();

    if half_turns.last().unwrap().contains("-") {
        half_turns.remove(half_turns.len()-1);
    }

    half_turns
}