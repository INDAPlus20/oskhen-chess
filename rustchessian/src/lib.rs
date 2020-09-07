#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod Units {
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
    struct square {
        Piece: Units::Pieces,
        Team: i8,
    }

    struct Board {
        matrix: [[square; 8]; 8],
    }

    pub fn init_board(){
    
        let mut board_matrix =  [[0; 8]; 8];
        println!("Board: {:?}", board_matrix);
    }
}

