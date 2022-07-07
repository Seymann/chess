use std::fmt;

use super::ChessMoveError;

struct Positions {
    val:  [[char; 8]; 8]
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
    fn insert(&mut self, val: char, coordinates: [usize; 2]) -> Result<(), ChessMoveError> {
        if coordinates[0] > 7 || coordinates[1] > 7 {
            return Err(ChessMoveError::InvalidCoordinates);
        }
        self.val[coordinates[0]][coordinates[1]] = val;
        Ok(())
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
    pub fn is_at(&self, a: usize, b: usize) -> char {
        return self.positions.at(a,b);
    }

    pub fn play_move(&mut self, move1: &String, move2: &String) -> Result<(), ChessMoveError> {
        let co1 = super::Game::move_to_coordinates(&move1)?;
        let co2 = super::Game::move_to_coordinates(&move2)?;
        let piece = self.is_at(co1[0], co1[1]);

        self.positions.insert(' ', co1);
        self.positions.insert(piece, co2);

        Ok(())

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