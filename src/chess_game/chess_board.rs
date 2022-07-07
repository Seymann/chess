use std::fmt;

use super::ChessMoveError;

struct Positions {
    val: [[char; 8]; 8]
}
 
impl Positions {
    fn new() -> Positions {
        Positions {
            val : 
            [['r','j','b','q','k','b','j','r'],
            ['p','p','p','p','p','p','p','p'],
            [' ',' ',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ',' ',' ',' ',' ',' '],
            [' ',' ',' ',' ',' ',' ',' ',' '],
            ['P','P','P','P','P','P','P','P'],
            ['R','J','B','Q','K','B','J','R']]
        }
    }
    fn from_board(board:[[char;8]; 8]) -> Positions {
        Positions {
            val :board
        }
    }
    fn at(&self, a: usize, b: usize) -> char {
        self.val[a][b]
    }
}

pub struct Board {
    positions: Positions,
    white_turn: bool
}

impl Board {
    pub fn new() -> Board{
        Board {
            positions: Positions::new(),
            white_turn: true
        }
    }
    pub fn is_at(&self, first: char, second: char) -> Result<char, ChessMoveError> {
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
                    Err(e) => return Err(ChessMoveError::InvalidCharUsed(String::from("Second number was not a Digit")))
                }
            },
            None => return Err(ChessMoveError::InvalidCharUsed(String::from("Second number was not a Digit")))
        }
        if y > 8 || y < 1 {
            return Err(ChessMoveError::InvalidCharUsed(String::from("Number not between 1-8")))
        }

        return Ok(self.positions.at(y,x));
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let mut turn: &str;
        if self.white_turn {
            turn = "white";
        } else {
            turn = "black";
        }

        write!(f, "{:?} \n{:?} \n{:?} \n{:?} \n{:?} \n{:?} \n{:?} \n{:?} \nTurn: {}", 
        self.positions.val[0],
        self.positions.val[1],
        self.positions.val[2],
        self.positions.val[3],
        self.positions.val[4],
        self.positions.val[5],
        self.positions.val[6],
        self.positions.val[7],
        turn)
    }
}