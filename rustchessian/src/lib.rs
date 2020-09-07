#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod board {
    pub fn init_board(){
        let mut board_matrix =  [[0; 8]; 8];
        println!("Board: {:?}", board_matrix);
    }
}

