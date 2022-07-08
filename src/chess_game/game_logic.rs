use super::ChessMoveError;
use super::Game;

pub fn validate_input(move1: &String) -> Result<(), ChessMoveError> {
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

pub fn move_to_coordinates(move1: &String) -> Result<[usize; 2], ChessMoveError> {
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
    let mut y;
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

pub fn validate_move(game: &Game, move1: &String, move2: &String) -> Result<(), ChessMoveError> {
    validate_input(move1)?;
    validate_input(move2)?;

    let coordinates1 = move_to_coordinates(&move1)?; 
    let coordinates2 = move_to_coordinates(&move2)?;

    let start_piece = game.board.is_at(coordinates1[0], coordinates1[1]);
    let end_point = game.board.is_at(coordinates2[0], coordinates2[1]);

    // check if move is correct

    // is correct piece being moved?
    if game.board.is_white_turn() && start_piece.is_lowercase() {
        return Err(ChessMoveError::WrongTurn(String::from("It is Whites turn, yet Black is being moved")))
    }
    if !game.board.is_white_turn() && start_piece.is_uppercase() {
        return Err(ChessMoveError::WrongTurn(String::from("It is Blacks turn, yet White is being moved")))
    }
    if start_piece == ' ' {
        return Err(ChessMoveError::InvalidStartPiece)
    }
    if (start_piece.is_uppercase() && end_point.is_uppercase()) || (start_piece.is_lowercase() && end_point.is_lowercase()) {
        return Err(ChessMoveError::TakingOwnPiece)
    }

    Ok(())
}
