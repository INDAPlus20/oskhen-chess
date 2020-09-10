use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn square_is_empty(){
        let empty_piece = Units::Piece {
            Rank: Units::Rank::Empty,
            Color: Units::Color::Empty,
        };
        let empty_square = Board::Square {
            piece: empty_piece,
            coordinate: (65, 65)
        };
        assert_eq!(empty_square.is_empty(), true);
    }
    #[test]
    fn square_is_not_empty(){
        let piece = Units::Piece {
            Rank: Units::Rank::Rook,
            Color: Units::Color::Black,
        };
        let square = Board::Square {
            piece: piece,
            coordinate: (65, 65)
        };
        assert_eq!(square.is_empty(), false);
    }
    //#[test] - Test not working?
    fn origin_correctly_placed() {
        let game = Board::BoardState::read(String::from("game"));
        let color = game.matrix[0][0].piece.Color;
        if let Units::Color::White = color {
            assert!(true);
        }
        assert!(false, "Expected white, got {:?}", color);
    }
}

pub mod Units {
    #[derive(Debug, Copy, Clone)]
    pub enum Rank {
        Empty,
        Pawn,
        Rook,
        Knight,
        Bishop,
        Queen,
        King,
    }
    #[derive(Debug, Copy, Clone)]
    pub enum Color {
        Empty,
        White,
        Black,
    }
    #[derive(Debug, Copy, Clone)]
    pub struct Piece {
        pub Rank: Rank,
        pub Color: Color,
    }

    pub fn init_empty_piece() -> Piece {
        Piece {
                Rank: Rank::Empty,
                Color: Color::Empty,
        }
    }   

    mod movements {
        use super::*;
        use crate::Board;

        impl Board::BoardState {
            pub fn r#move(&self, square: Board::Square) {
                let rank = square.piece.Rank;
                let coordinates = square.coordinate;
                let moveset = match rank {
                    Empty => panic!("Tried to move empty square!"),
                    Pawn => move_pawn(coordinates, self),
                    Rook => move_rook(coordinates, self),
                    Knight => move_knight(coordinates, self),
                    Bishop => move_bishop(coordinates, self),
                    Queen => move_queen(coordinates, self),
                    King => move_king(coordinates, self),
                };
            }
        }

        pub fn move_pawn(coordinates: (usize, usize), gamestate: &Board::BoardState) {
            let x = coordinates.0;
            let y = coordinates.1;
            let this_square = gamestate.matrix[x][y];
            let team = this_square.piece.Color;
            let available_moves: Vec<(isize, isize)> = Vec::new();

            

        }
        pub fn move_rook(coordinates: (usize, usize), gamestate: &Board::BoardState) {
            let available_moves: Vec<(isize, isize)> = Vec::new();

        }
        pub fn move_knight(coordinates: (usize, usize), gamestate: &Board::BoardState) {
            let available_moves: Vec<(isize, isize)> = Vec::new();
            
        }
        pub fn move_bishop(coordinates: (usize, usize), gamestate: &Board::BoardState) {
            let available_moves: Vec<(isize, isize)> = Vec::new();

        }
        pub fn move_queen(coordinates: (usize, usize), gamestate: &Board::BoardState) {
            let available_moves: Vec<(isize, isize)> = Vec::new();

        }
        pub fn move_king(coordinates: (usize, usize), gamestate: &Board::BoardState) {
            let available_moves: Vec<(isize, isize)> = Vec::new();

        }
    }

}

pub mod Board {

    use super::Units;
    use std::{fmt, fs};

    #[derive(Debug, Copy, Clone)]
    pub struct Square {
        pub piece: Units::Piece,
        pub coordinate: (usize, usize),
    }
    #[derive(Copy, Clone)]
    pub struct BoardState {
        pub matrix: [[Square; 8]; 8],
    }

    impl BoardState {

        fn init_empty_square() -> Square {
            let piece = Units::init_empty_piece();
            Square{
                piece: piece,
                coordinate: (65, 65),
            }
        }

        fn blockstate_to_piece(object: &str) -> Units::Piece {
            let empty_piece = Units::init_empty_piece();
            if object.eq("XX") {
                return empty_piece;
            }

            let rank = match object.chars().nth(0).unwrap() {
                'P' => Units::Rank::Pawn,
                'R' => Units::Rank::Rook,
                'N' => Units::Rank::Knight,
                'B' => Units::Rank::Bishop,
                'Q' => Units::Rank::Queen,
                'K' => Units::Rank::King,
                _ => panic!("Piece signature not valid!")
            };

            let color = match object.chars().nth(1).unwrap() {
                'B' => Units::Color::Black,
                'W' => Units::Color::White,
                _ => panic!("Color signature not valid!")
            };

            Units::Piece {
                    Rank: rank,
                    Color: color,
                }
        }

        pub fn new() -> BoardState {
            let empty_square = BoardState::init_empty_square();
            let mut init_matrix = [[empty_square; 8]; 8];
            for team in 0..2 { //dw, will remove
                init_matrix[team * 7][0].piece.Rank = Units::Rank::Rook;
                init_matrix[team * 7][1].piece.Rank = Units::Rank::Knight;
                init_matrix[team * 7][2].piece.Rank = Units::Rank::Bishop;
                init_matrix[team * 7][3].piece.Rank = Units::Rank::Queen;
                init_matrix[team * 7][4].piece.Rank = Units::Rank::King;
                init_matrix[team * 7][5].piece.Rank = Units::Rank::Bishop;
                init_matrix[team * 7][6].piece.Rank = Units::Rank::Knight;
                init_matrix[team * 7][7].piece.Rank = Units::Rank::Rook;
            }

            for i in 0..8 {
                init_matrix[0][i].piece.Color = Units::Color::Black;
                init_matrix[1][i].piece = Units::Piece {
                    Rank: Units::Rank::Pawn,
                    Color: Units::Color::Black,
                };
            }
            for i in 0..8 {
                init_matrix[7][i].piece.Color = Units::Color::White;
                init_matrix[6][i].piece = Units::Piece {
                    Rank: Units::Rank::Pawn,
                    Color: Units::Color::White,
                };
            }

            BoardState {
                matrix: init_matrix,
            }

        }
        pub fn read(filename: String) -> BoardState {
            let contents = fs::read_to_string(filename).expect("Panic at reading file"); //TODO: Error handling
            let contents = contents.replace("\n", " ");

            let mut string_objects: Vec<&str> = contents.trim().split(" ").collect();
            string_objects.reverse();

            let boardsize = string_objects.len();
            if boardsize != 64 {
                panic!("Invalid gamestate file: expected 64 squares, got {}", boardsize)
            }

            let mut piece_objects =  Vec::<Units::Piece>::new();
            let mut square_matrix = [[BoardState::init_empty_square(); 8]; 8];

            for object in string_objects{
                let this_piece = BoardState::blockstate_to_piece(object);
                piece_objects.push(this_piece);
            }

            for line in 0..8{
                for block in 0..8{
                    let square: Square = Square {
                        piece: piece_objects[8*line + block],
                        coordinate: (line, block),
                    };
                    square_matrix[line][block] = square;
                }
            }
            // Transform contents to 64 objects, split 8x8. Then 1:1 transform to Boardstate

            BoardState {
                matrix: square_matrix,
            }

        }
    }

    impl Square {
        pub fn is_empty(&self) -> bool {
            if let Units::Rank::Empty = self.piece.Rank {
                return true;
            }
            false
        }
    }

    impl fmt::Display for Square {  
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.is_empty() {
                return write!(f, "{}", "_");
            } else {
                return write!(f, "{}", self.piece.Rank as i32);
            }
        }
    }

    impl fmt::Display for BoardState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut formatted_string = String::new();
            for line in self.matrix.iter() {
                for entry in line {
                    formatted_string.push_str(&String::from(format!("{} ", entry)));
                }
                formatted_string.push_str(&String::from(format!("\n")));
            }
            let output: String = formatted_string.chars().rev().collect();
            write!(f, "{}", output)
        }
    }
}
