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
    pub enum Rank{
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
    pub struct Piece{
        pub Rank: Rank,
        pub Color: Color,
    }

}

pub mod board {

use super::Units;
use std::fmt;

    #[derive(Debug, Copy, Clone)]
    pub struct Square {
        Piece: Units::Piece,
    }
    #[derive(Copy, Clone)]
    pub struct Board {
        matrix: [[Square; 8]; 8],
    }

    impl Board {
        pub fn new() -> Board {

            let empty_square: Square = Square{
                Piece: Units::Piece{
                    Rank: Units::Rank::Empty,
                    Color: Units::Color::Empty,
                }
            };

            let mut init_matrix = [[empty_square; 8]; 8];
            for team in 0..2{
                init_matrix[team*7][0].Piece.Rank = Units::Rank::Rook;
                init_matrix[team*7][1].Piece.Rank = Units::Rank::Knight;
                init_matrix[team*7][2].Piece.Rank = Units::Rank::Bishop;
                init_matrix[team*7][3].Piece.Rank = Units::Rank::Queen;
                init_matrix[team*7][4].Piece.Rank = Units::Rank::King;
                init_matrix[team*7][5].Piece.Rank = Units::Rank::Bishop;
                init_matrix[team*7][6].Piece.Rank = Units::Rank::Knight;
                init_matrix[team*7][7].Piece.Rank = Units::Rank::Rook;
            }

            for i in 0..8{
                init_matrix[0][i].Piece.Color = Units::Color::Black;
                init_matrix[1][i].Piece = Units::Piece{
                    Rank: Units::Rank::Pawn,
                    Color: Units::Color::Black,
                };
            }
            for i in 0..8{
                init_matrix[7][i].Piece.Color = Units::Color::White;
                init_matrix[6][i].Piece = Units::Piece{
                    Rank: Units::Rank::Pawn,
                    Color: Units::Color::White,
                };
            }

            let init_board: Board = Board{
                matrix: init_matrix,
            };

            init_board
        }
    }

    impl Square {
        fn is_empty(&self) -> bool {
            if let Units::Rank::Empty = self.Piece.Rank{
                return true;
            }
            false
        }
    }

    impl fmt::Display for Square {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            if self.is_empty(){
                return write!(f, "{}", "_");
            }
            else {
                return write!(f, "{}", self.Piece.Rank as i32);
            }
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
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
