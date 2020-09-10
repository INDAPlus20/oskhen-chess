use std::fmt;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
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

    impl Piece {
        pub fn r#move(&self) {
            let rank = self.Rank;
            let moveset = match rank {
                Empty => panic!("Tried to move empty square!"),
                Pawn => movements::move_pawn(self),
                Rook => movements::move_rook(self),
                Knight => movements::move_knight(self),
                Bishop => movements::move_bishop(self),
                Queen => movements::move_queen(self),
                King => movements::move_king(self),
            };
        }
    }

    mod movements {
        use super::*;
        pub fn move_pawn(unit: &Piece) {

        }
        pub fn move_rook(unit: &Piece) {

        }
        pub fn move_knight(unit: &Piece) {

        }
        pub fn move_bishop(unit: &Piece) {

        }
        pub fn move_queen(unit: &Piece) {

        }
        pub fn move_king(unit: &Piece) {

        }
    }

}

pub mod Board {

    use super::Units;
    use std::{fmt, fs, convert::TryInto};

    #[derive(Debug, Copy, Clone)]
    pub struct Square {
        piece: Units::Piece,
        coordinate: (isize, isize),
    }
    #[derive(Copy, Clone)]
    pub struct BoardState {
        matrix: [[Square; 8]; 8],
    }

    impl BoardState {

        fn init_empty_square() -> Square {
            let piece = Units::init_empty_piece();
            Square{
                piece: piece,
                coordinate: (-1, -1),
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

            let string_objects: Vec<&str> = contents.trim().split(" ").collect();

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
                        coordinate: (line.try_into().unwrap(), block.try_into().unwrap()),
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
        fn is_empty(&self) -> bool {
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
            write!(f, "{}", formatted_string)
        }
    }
}
