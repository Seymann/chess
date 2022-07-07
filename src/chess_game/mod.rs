pub mod chess_board;

use core::num;

use text_io::scan;
#[derive(Debug)]
pub enum ChessMoveError {
    InvalidCharUsed(String),
    MoveTooLong(String),
    InvalidCoordinates
}

pub struct Game {
    board: chess_board::Board
}

impl Game {
    pub fn start_game() {
        let mut game = Game {
            board: chess_board::Board::new()
        };

        let mut move1: String;
        let mut move2: String;
        loop {
            println!("{}", game.board);
            println!("Please enter your move!\nIn the Format \"e4 c6\".\nTo Leave type \"q q\"");
            scan!("{} {}", move1, move2);
            move1.retain(|c| !c.is_whitespace());
            move2.retain(|c| !c.is_whitespace());
            if move1.eq("q") && move2.eq("q") {
                break;
            }

            if let Err(e) = game.validate_move(&move1, &move2) {
                println!("{:?}", e );
                continue;
            }
            if let Err(e) = game.board.play_move(&move1, &move2) {
                println!("{:?}", e);
                continue;
            }
            
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

    fn move_to_coordinates(move1: &String) -> Result<[usize; 2], ChessMoveError> {
        let vec: Vec<char> = move1.chars().collect();
        if vec.len() != 2 {
            return Err(ChessMoveError::MoveTooLong(String::from("Move not 2 letters long.")));
        }
        let first = vec[0];
        let second = vec[1];
        let mut x = 8;
        match first {
            'a' => x = 0,
            'b' => x = 1,
            'c' => x = 2,
            'd' => x = 3,
            'e' => x = 4,
            'f' => x = 5,
            'g' => x = 6,
            'h' => x = 7,
            _ => {}
        }
        if x == 8 {
            return Err(ChessMoveError::InvalidCharUsed(String::from("first letter was not between a-h")));
        }
        let mut y = 9;
        match second.to_digit(10) {
            Some(num) => {
                match num.try_into() {
                    Ok(num) => y = num,
                    Err(_e) => return Err(ChessMoveError::InvalidCharUsed(String::from("Second number was not a Digit")))
                }
            },
            None => return Err(ChessMoveError::InvalidCharUsed(String::from("Second number was not a Digit")))
        }
        y = 8 - y;
        if y >= 8 {
            return Err(ChessMoveError::InvalidCharUsed(String::from("Number not between 1-8")))
        }
        Ok([y,x])
    }

    fn validate_move(&self, move1: &String, move2: &String) -> Result<(), ChessMoveError> {
        Game::validate_input(move1)?;
        Game::validate_input(move2)?;
        let char_vec: Vec<char> = move1.chars().collect();
        let char_vec2: Vec<char> = move2.chars().collect();

        let coordinates1 = Game::move_to_coordinates(&move1)?; 
        let coordinates2 = Game::move_to_coordinates(&move2)?;

        let start_piece = self.board.is_at(coordinates1[0], coordinates1[1]);
        let end_point = self.board.is_at(coordinates2[0], coordinates2[1]);

        // check if move is correct

        //println!("{}, {}", start_piece, end_point);

        Ok(())
    }


}
        