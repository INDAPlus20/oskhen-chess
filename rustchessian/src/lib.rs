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
}

pub mod Board {

    use super::Units;
    use std::{fmt, fs};

    #[derive(Debug, Copy, Clone)]
    pub struct Square {
        Piece: Units::Piece,
    }
    #[derive(Copy, Clone)]
    pub struct BoardState {
        matrix: [[Square; 8]; 8],
    }

    impl BoardState {

        fn init_empty() -> Square {
            Square {
                Piece: Units::Piece {
                    Rank: Units::Rank::Empty,
                    Color: Units::Color::Empty,
                },
            }
        }

        fn blockstate_to_square(object: &str) -> Square {
            let empty_square = BoardState::init_empty();
            if object.eq("XX") {
                return empty_square;
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

            Square {
                Piece: Units::Piece {
                    Rank: rank,
                    Color: color,
                },
            }
        }


        pub fn new() -> BoardState {

            let empty_square = BoardState::init_empty();
            let mut init_matrix = [[empty_square; 8]; 8];
            for team in 0..2 { //dw, will remove
                init_matrix[team * 7][0].Piece.Rank = Units::Rank::Rook;
                init_matrix[team * 7][1].Piece.Rank = Units::Rank::Knight;
                init_matrix[team * 7][2].Piece.Rank = Units::Rank::Bishop;
                init_matrix[team * 7][3].Piece.Rank = Units::Rank::Queen;
                init_matrix[team * 7][4].Piece.Rank = Units::Rank::King;
                init_matrix[team * 7][5].Piece.Rank = Units::Rank::Bishop;
                init_matrix[team * 7][6].Piece.Rank = Units::Rank::Knight;
                init_matrix[team * 7][7].Piece.Rank = Units::Rank::Rook;
            }

            for i in 0..8 {
                init_matrix[0][i].Piece.Color = Units::Color::Black;
                init_matrix[1][i].Piece = Units::Piece {
                    Rank: Units::Rank::Pawn,
                    Color: Units::Color::Black,
                };
            }
            for i in 0..8 {
                init_matrix[7][i].Piece.Color = Units::Color::White;
                init_matrix[6][i].Piece = Units::Piece {
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

            let mut square_objects =  Vec::<Square>::new();
            let mut square_matrix = [[BoardState::init_empty(); 8]; 8];

            for object in string_objects{
                let this_square = BoardState::blockstate_to_square(object);
                square_objects.push(this_square);
            }

            for line in 0..8{
                for block in 0..8{
                    square_matrix[line][block] = square_objects[8*line + block];
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
            if let Units::Rank::Empty = self.Piece.Rank {
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
                return write!(f, "{}", self.Piece.Rank as i32);
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
