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
    pub enum Pieces{
        Empty,
        Pawn,
        Rook,
        Knight,
        Bishop,
        Queen,
        King,
    }
}

pub mod board {

use super::Units;
use std::fmt;
    #[derive(Debug, Copy, Clone)]
    enum Color {
        Empty,
        White,
        Black,
    }
    #[derive(Debug, Copy, Clone)]
    struct Square {
        Piece: Units::Pieces,
        Team: Color,
    }
    #[derive(Copy, Clone)]
    pub struct Board {
        matrix: [[Square; 8]; 8],
    }

    impl Board {
        pub fn new() -> Board {
            let empty_square: Square = Square{
                Piece: Units::Pieces::Empty,
                Team: Color::Empty,
            };

            Board {
                matrix: [[empty_square; 8]; 8],
            }
        }
    }

    impl Square {
        fn is_empty(&self) -> bool {
            if let Units::Pieces::Empty = self.Piece{
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
                return write!(f, "{}", self.Piece as i32);
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
