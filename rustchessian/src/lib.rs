#![allow(dead_code)] // No annoying warnings

use std::{convert::TryInto, fmt};
use std::io::{self, BufRead};


#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug, Copy, Clone)]
struct Square {
    piece: Option<Piece>,
    coordinate: (isize, isize),
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Team {
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
enum Actiontype {
    Regular,
    Castling,
    EnPassant,
    Promotion,
}

#[derive(Debug, Copy, Clone)]
pub struct Action {
    from: Square,
    to: Square,
    movetype: Actiontype,
}

#[derive(Debug, Copy, Clone)]
struct Piece {
    team: Team,
    rank: Rank,
}
#[derive(Debug, Copy, Clone)]
enum Rank {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct Game {
    grid: [[Square; 8]; 8],
    player: Team,
    history: Vec<Action>,
}

fn valid_coordinates(x: isize, y:isize) -> bool {
    if x < 0 || x > 7 || y < 0 || y > 7{
        return false;
    }
    true
}

fn not_same_team(team: Team, square: Square) -> bool {
    if square.piece.is_some(){
        if square.piece.unwrap().team != team{
            return true;
        }
    }
    false
}

fn string_from_coordinates(coordinates: (isize, isize)) -> String {
    let mut position = String::new();
    let column = match coordinates.0 {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => panic!("Invalid coordinate!"),
    };
    position.push(column);
    let row = (coordinates.1 + 1).to_string();
    position.push_str(&row);
    position
}

fn promotion_prompt() -> Rank {
    println!("Choose which piece to promote your pawn to:");
    println!("1. Queen");
    println!("2. Rook");
    println!("3. Bishop");
    println!("4. Knight");
    loop {
        let input_index = io::stdin().lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
        let choice = match input_index {
            1 => Rank::Queen,
            2 => Rank::Rook,
            3 => Rank::Bishop,
            4 => Rank::Knight,
            _ => {
                println!("Please choose a valid option: ");
                continue
            }
        };
        return choice;
    }
}

impl Game {

    fn toggle_team(&mut self) {
        self.player = match self.player {
            Team::White => Team::Black,
            Team::Black => Team::White,
        };
    }

    // Set target to origin, origin to empty. Handle captures, add to history. Change player turn.
    pub fn make_move(&mut self, action: Action) {
        let target = action.to;
        let origin = action.from;
        let movetype = action.movetype;

        match movetype {
            Actiontype::Regular => {
                self.grid[target.coordinate.0 as usize][target.coordinate.1 as usize] = Square {
                    piece: origin.piece,
                    coordinate: target.coordinate
                };
        
                self.grid[origin.coordinate.0 as usize][origin.coordinate.1 as usize] = Square {
                    piece: None,
                    coordinate: origin.coordinate
                };
            }
            Actiontype::EnPassant => {
                self.grid[target.coordinate.0 as usize][target.coordinate.1 as usize] = Square {
                    piece: origin.piece,
                    coordinate: target.coordinate
                };
        
                self.grid[origin.coordinate.0 as usize][origin.coordinate.1 as usize] = Square {
                    piece: None,
                    coordinate: origin.coordinate
                };

                self.grid[target.coordinate.0 as usize][origin.coordinate.1 as usize] = Square {
                    piece: None,
                    coordinate: (target.coordinate.0, origin.coordinate.1),
                };
            }
            Actiontype::Promotion => {
                let new_rank = promotion_prompt();
                let new_piece: Piece = Piece {
                    team: self.player,
                    rank: new_rank,
                };
                self.grid[target.coordinate.0 as usize][target.coordinate.1 as usize] = Square {
                    piece: Some(new_piece),
                    coordinate: target.coordinate,
                };
                self.grid[origin.coordinate.0 as usize][origin.coordinate.1 as usize] = Square {
                    piece: None,
                    coordinate: origin.coordinate
                };
            },
            Actiontype::Castling => {
                self.grid[origin.coordinate.0 as usize][origin.coordinate.1 as usize] = Square {
                    piece: None,
                    coordinate: origin.coordinate
                };
                self.grid[target.coordinate.0 as usize][target.coordinate.1 as usize] = Square {
                    piece: origin.piece,
                    coordinate: target.coordinate,
                };
                if target.coordinate.0 == 6 {
                    self.grid[(target.coordinate.0 - 1 )as usize][target.coordinate.1 as usize] = Square {
                        piece: Some(Piece {
                            team: self.player,
                            rank: Rank::Rook,
                        }),
                        coordinate: ((target.coordinate.0 - 1), (target.coordinate.1))
                    };

                   self.grid[7][target.coordinate.1 as usize] = Square {
                       piece: None,
                       coordinate: (7, target.coordinate.1),
                   };
                } else {
                    self.grid[(target.coordinate.0 + 1 )as usize][target.coordinate.1 as usize] = Square {
                        piece: Some(Piece {
                            team: self.player,
                            rank: Rank::Rook,
                        }),
                        coordinate: ((target.coordinate.0 + 1), (target.coordinate.1))
                    };

                   self.grid[0][target.coordinate.1 as usize] = Square {
                       piece: None,
                       coordinate: (0, target.coordinate.1),
                }
            }
        };



        self.toggle_team();
        self.history.push(action);

    }

    pub fn gen_move_from_string(&self, coordinate: &str) -> Vec<Action> {
        let square = self.square_from_string(coordinate);
        let moveset = self.generate_moves(square);
        for (index, movement) in moveset.iter().enumerate() {
            println!("{}. {}", index+1, movement);
        }
        return moveset;
    }

    fn square_from_string(&self, coordinate: &str) -> Square {
        if coordinate.len() != 2 {
            panic!("Invalid coordinate")
        }
        let column: usize = match coordinate.chars().nth(0).unwrap().to_ascii_lowercase() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("invalid coordinate")
        };
        let row: usize = coordinate.chars().nth(1).unwrap().to_digit(10).expect("Invalid coordinate!") as usize;
        let this_square = Square {
            piece: self.grid[column][row-1].piece,
            coordinate: self.grid[column][row-1].coordinate,
        };
        this_square
    }

    fn generate_moves(&self, square: Square) -> Vec<Action> {
        let piece = match square.piece {
            Some(i) => i,
            None => panic!("Tried to move empty square!"),
        };

        if self.player != piece.team {
            panic!("Cannot move enemy piece!")
        }

        let moveset: Vec<Action> = match piece.rank {
            Rank::Pawn => self.gen_moveset_pawn(square),
            Rank::Rook => self.gen_moveset_rook(square),
            Rank::Knight => self.gen_moveset_knight(square),
            Rank::Bishop => self.gen_moveset_bishop(square),
            Rank::Queen => self.gen_moveset_queen(square),
            Rank::King => self.gen_moveset_king(square),
        };
        moveset
    }

    fn try_add_move(&self, old_square: Square, new_x: isize, new_y: isize) -> Option<Action> {

        if valid_coordinates(new_x, new_y){ 
            let new_square = self.grid[new_x as usize][new_y as usize];

            if new_square.piece.is_none() {
                let this_action = Action {
                    from: old_square,
                    to: new_square,
                    movetype: Actiontype::Regular
                };
                return Some(this_action);
            }
            else if not_same_team(self.player, new_square) {
                let this_action = Action {
                    from: old_square,
                    to: new_square,
                    movetype: Actiontype::Regular
                };
               return Some(this_action);
            }
        }
        return None
        
    }

    fn gen_scaled_moveset_from_offset(&self, this_square: Square, offsets: [(isize, isize); 4]) -> Vec<Action> {
        let mut available_moves = Vec::<Action>::new();
        let x = this_square.coordinate.0;
        let y = this_square.coordinate.1; 
        for offset in offsets.iter() {
            let dx = offset.0;
            let dy = offset.1;
            let mut scalar = 1;
            loop {
                let new_x = x + dx*scalar;
                let new_y = y + dy*scalar;

                if valid_coordinates(new_x, new_y) {
                    let new_square: Square = self.grid[new_x as usize][new_y as usize];
                    if new_square.piece.is_none() {
                        let this_action = Action {
                            from: this_square,
                            to: new_square,
                            movetype: Actiontype::Regular
                        };
                        available_moves.push(this_action);
                    }
                    else if not_same_team(self.player, new_square) {
                        let this_action = Action {
                            from: this_square,
                            to: new_square,
                            movetype: Actiontype::Regular
                        };
                        available_moves.push(this_action);
                        break;
                    }
                    else {
                        break;
                    }
                }
                else {
                    break;
                }
                scalar += 1;
            }
        }

        available_moves
    }

    fn gen_moveset_diagonal(&self, this_square: Square) -> Vec<Action> {

        let offsets = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        self.gen_scaled_moveset_from_offset(this_square, offsets)
        
    }

    fn gen_moveset_straight(&self, this_square: Square) -> Vec<Action> {

        let offsets = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        self.gen_scaled_moveset_from_offset(this_square, offsets)

    }

    fn gen_moveset_pawn(&self, this_square: Square) -> Vec<Action> {
        let mut available_moves = Vec::<Action>::new();
        let offset: isize = match self.player {
            Team::White => 1,
            Team::Black => -1,
        };
        let x = this_square.coordinate.0;
        let y = this_square.coordinate.1; 
        let y_forward = y + offset;

        if valid_coordinates(x, y_forward){
            let new_square: Square = self.grid[x as usize][y_forward as usize];
            if new_square.piece.is_none() {
                let this_action = Action {
                    from: this_square,
                    to: new_square,
                    movetype: Actiontype::Regular
                };
                available_moves.push(this_action);
            }
        }

        if valid_coordinates(x+1, y_forward) {
            let new_square: Square = self.grid[(x+1) as usize][y_forward as usize];
            if not_same_team(self.player, new_square) {
                let this_action = Action {
                    from: this_square,
                    to: new_square,
                    movetype: Actiontype::Regular
                };
                available_moves.push(this_action);
            }
        }

        if valid_coordinates(x-1, y_forward) {
            let new_square: Square = self.grid[(x-1) as usize][y_forward as usize];
            if not_same_team(self.player, new_square) {
                let this_action = Action {
                    from: this_square,
                    to: new_square,
                    movetype: Actiontype::Regular
                };
                available_moves.push(this_action);
            }
        }

        if y == 1 && self.player == Team::White || y == 6 && self.player == Team::Black {
            let y_double_forward = y + (offset*2);
            if valid_coordinates(x, y_double_forward) {
                let new_square: Square = self.grid[x as usize][y_double_forward as usize];
                if new_square.piece.is_none() {
                    let this_action = Action {
                        from: this_square,
                        to: new_square,
                        movetype: Actiontype::Regular
                    };
                    available_moves.push(this_action);
                }
            }
        }

        let last_move = self.history.last();
        if last_move.is_some() {
            let last_move = last_move.unwrap();
            match last_move.from.piece.unwrap().rank {
                Rank::Pawn => if (last_move.to.coordinate.1 - last_move.from.coordinate.1).abs() == 2 {
                    if last_move.to.coordinate.1 == this_square.coordinate.1 {
                        let mut new_square = last_move.to;
                        new_square.coordinate.1 += offset;
                        let this_action = Action {
                            from: this_square,
                            to: new_square,
                            movetype: Actiontype::EnPassant,
                        };
                        available_moves.push(this_action);
                    }
                }
                _ => (),
            };
        }   

        let mut return_moves = Vec::<Action>::new();
        
        for action in available_moves.iter(){
            let this_action = action.to_owned();
            if action.to.coordinate.1 == 0 || action.to.coordinate.1 == 7 {
                let this_action = Action {
                    from: action.from,
                    to: action.to,
                    movetype: Actiontype::Promotion,
                };
                return_moves.push(this_action);
            } else {
                return_moves.push(this_action);
            }
        }

        return_moves

    }

    fn gen_moveset_rook(&self, this_square: Square) -> Vec<Action> {
        self.gen_moveset_straight(this_square)
    }

    fn gen_moveset_knight(&self, this_square: Square) -> Vec<Action> {
        let mut available_moves = Vec::<Action>::new();
        let x = this_square.coordinate.0;
        let y = this_square.coordinate.1;

        let offsets = [(1, 2), (2, 1)];
        let scalars = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        for offset in offsets.iter() {
            for scalar in scalars.iter() {
                let new_x = x + offset.0*scalar.0;
                let new_y = y + offset.1*scalar.1;
                match self.try_add_move(this_square, new_x, new_y) {
                    None => (),
                    Some(action) => available_moves.push(action),
                }
            }
        }

        available_moves
    }

    fn gen_moveset_bishop(&self, this_square: Square) -> Vec<Action> {
        self.gen_moveset_diagonal(this_square)
    }

    fn gen_moveset_queen(&self, this_square: Square) -> Vec<Action> {
        let mut available_moves = Vec::<Action>::new();
        let straight = self.gen_moveset_straight(this_square);
        let diagonal = self.gen_moveset_diagonal(this_square);
        available_moves.extend(straight);
        available_moves.extend(diagonal);
        available_moves
        
    }

    fn gen_moveset_king(&self, this_square: Square) -> Vec<Action> {
        let mut available_moves = Vec::<Action>::new();
        let x = this_square.coordinate.0;
        let y = this_square.coordinate.1;

        let offsets = [(0, 1), (1, 0), (0, -1), (-1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];

        for offset in offsets.iter(){
            let new_x = x + offset.0;
            let new_y = y + offset.1;
            match self.try_add_move(this_square, new_x, new_y) {
                None => (),
                Some(action) => available_moves.push(action),
            }
        }

        let mut has_king_moved = false;
        for action in self.history.iter() {
            match action.from.piece.unwrap().rank {
                Rank::King => has_king_moved = true,
                _ => ()
            };
        }
        if !has_king_moved {
            //Check left rook
            let mut left_rook_flag = true;
            let mut new_x = x-1;
            while x > 0 {
                if self.grid[new_x as usize][y as usize].piece.is_some() {
                    left_rook_flag = false;
                }
                new_x-=1;
            }
            if left_rook_flag{
                for action in self.history.iter() {
                    match action.from.piece.unwrap().rank {
                        Rank::Rook => if action.from.coordinate.0 == 0 {
                            left_rook_flag = false;
                        }
                        _ => (),
                    }
                }
            }
            if left_rook_flag {
                let new_square = Square {
                    piece: this_square.piece,
                    coordinate: (2, y),
                };
                let this_action = Action {
                    from: this_square,
                    to: new_square,
                    movetype: Actiontype::Castling,
                };
                available_moves.push(this_action);
            }

            //Check right rook
            let mut right_rook_flag = true;
            let mut new_x = x+1;
            while x < 7 {
                if self.grid[new_x as usize][y as usize].piece.is_some() {
                    right_rook_flag = false;
                }
                new_x+=1;
            }
            if right_rook_flag{
                for action in self.history.iter() {
                    match action.from.piece.unwrap().rank {
                        Rank::Rook => if action.from.coordinate.0 == 7 {
                            right_rook_flag = false;
                        }
                        _ => (),
                    }
                }
            }
            if right_rook_flag {
                let new_square = Square {
                    piece: this_square.piece,
                    coordinate: (6, y),
                };
                let this_action = Action {
                    from: this_square,
                    to: new_square,
                    movetype: Actiontype::Castling,
                };
                available_moves.push(this_action);
            }

        }

        available_moves
    }

    fn blockstate_to_piece(object: &str) -> Option<Piece> {
        if object.eq("XX") {
            return None;
        }

        let rank = match object.chars().nth(0).unwrap() {
            'P' => Rank::Pawn,
            'R' => Rank::Rook,
            'N' => Rank::Knight,
            'B' => Rank::Bishop,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            _ => panic!("Piece signature not valid!"),
        };

        let team = match object.chars().nth(1).unwrap() {
            'B' => Team::Black,
            'W' => Team::White,
            _ => panic!("Color signature not valid!"),
        };

        let piece = Piece {
            rank: rank,
            team: team,
        };

        return Some(piece);
    }

    pub fn new() -> Game {
        let blockstates: Vec<&str> = "RB NB BB KB QB BB NB RB
        PB PB PB PB PB PB PB PB
        XX XX XX XX XX XX XX XX
        XX XX XX XX XX XX XX XX
        XX XX XX XX XX XX XX XX
        XX XX XX XX XX XX XX XX
        PW PW PW PW PW PW PW PW
        RW NW BW KW QW BW NW RW"
            .trim()
            .split_whitespace()
            .rev()
            .collect();

        let placeholder_square = Square {
            // Fix array initalization to not require this workaround!
            piece: None,
            coordinate: (-1, -1),
        };

        let mut empty_grid: [[Square; 8]; 8] = [[placeholder_square; 8]; 8];

        let mut piece_objects = Vec::<Option<Piece>>::new();

        for object in blockstates {
            let this_piece = Game::blockstate_to_piece(object);
            piece_objects.push(this_piece);
        }
        for row in 0..8 {
            for column in 0..8 {
                let this_square: Square = Square {
                    piece: piece_objects[8 * row + column],
                    coordinate: (column.try_into().unwrap(), row.try_into().unwrap()),
                };
                empty_grid[column][row] = this_square;
            }
        }
        Game {
            grid: empty_grid,
            player: Team::White,
            history: Vec::<Action>::new(),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = self.piece;

        let type_of_piece = match piece {
            None => return write!(f, "{}", "_"),
            Some(type_of_piece) => type_of_piece,
        };

        let print: &str;

        if let Team::White = type_of_piece.team {
            print = match type_of_piece.rank {
                Rank::Pawn => "♙",
                Rank::Knight => "♘",
                Rank::Bishop => "♗",
                Rank::Rook => "♖",
                Rank::Queen => "♕",
                Rank::King => "♔",
            };
        } else {
            print = match type_of_piece.rank {
                Rank::Pawn => "♟︎",
                Rank::Knight => "♞",
                Rank::Bishop => "♝",
                Rank::Rook => "♜",
                Rank::Queen => "♛",
                Rank::King => "♚",
            };
        }
        return write!(f, "{}", print);
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut formatted_string = String::from("A B C D E F G H\n");
        for row in (0..8).rev() {
            for column in 0..8 {
                let entry = self.grid[column][row];
                formatted_string.push_str(&String::from(format!("{} ", entry)));
            }
            formatted_string.push_str(&String::from((row+1).to_string()));
            formatted_string.push_str(&String::from(format!("\n")));
        }
        write!(f, "{}", formatted_string)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let coordinate = string_from_coordinates(self.to.coordinate);
        write!(f, "{}", coordinate) 
    }
}

