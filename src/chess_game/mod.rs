pub mod chess_board;

use core::num;

use text_io::scan;
#[derive(Debug)]
pub enum ChessMoveError {
    InvalidCharUsed(String),
    MoveTooLong(String)
}

pub struct Game {
    board: chess_board::Board
}

impl Game {
    pub fn start_game() {
        let game = Game {
            board: chess_board::Board::new()
        };

        let mut move1: String;
        let mut move2: String;
        let mut move1_val: bool = false;
        let mut move2_val: bool = false;
        loop {
            println!("{}", game.board);
            println!("Please enter your move!\nIn the Format \"e4 c6\"");
            scan!("{} {}", move1, move2);
            game.validate_move(&move1, &move2);
        }

    }

    fn validate_input(move1: &String) -> Result<(), ChessMoveError> {
        for (i, c) in move1.char_indices() {
            if i == 0 {
                if c != 'a' && c != 'b' && c != 'c' && c != 'd' && c != 'e' && c != 'f' && c != 'g' && c != 'h' {
                    return Err(ChessMoveError::InvalidCharUsed(String::from("First letter was not between a-h")));
                }
            } 
            if i == 1 {
                let op_c = c.to_digit(10);
                match op_c {
                    Some(num_c) => {
                        if num_c > 8 || num_c < 1 {
                            return Err(ChessMoveError::InvalidCharUsed(String::from("Number was not between 1-8")));
                        }
                    },
                    None => return Err(ChessMoveError::InvalidCharUsed(String::from("Second letter was not a number")))
                }  
            }
            if i > 1 {
                return Err(ChessMoveError::MoveTooLong(String::from("Move was more than 2 Letters")));
            }
        }
        return Ok(());
    }

    fn validate_move(&self, move1: &String, move2: &String) -> Result<(), ChessMoveError> {
        Game::validate_input(move1)?;
        Game::validate_input(move2)?;
        let char_vec: Vec<char> = move1.chars().collect();
        let char_vec2: Vec<char> = move2.chars().collect();
        let start_piece = self.board.is_at(char_vec[0], char_vec[1])?;
        let end_point = self.board.is_at(char_vec2[0], char_vec2[1])?;
        println!("{}, {}", start_piece, end_point);

        Ok(())
    }
}
        