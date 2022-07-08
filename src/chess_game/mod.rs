pub mod chess_board;
pub mod game_logic;



use text_io::scan;
#[derive(Debug)]
pub enum ChessMoveError {
    InvalidCharUsed(String),
    MoveTooLong(String),
    InvalidCoordinates,
    WrongTurn(String),
    InvalidStartPiece,
    TakingOwnPiece
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
        println!("{}", game.board);
        loop {
            println!("Please enter your move!\nIn the Format \"e4 c6\".\nTo Leave type \"q q\"");
            scan!("{} {}", move1, move2);
            move1.retain(|c| !c.is_whitespace());
            move2.retain(|c| !c.is_whitespace());
            if move1.eq("q") && move2.eq("q") {
                break;
            }
            if let Err(e) = game_logic::validate_move(&game, &move1, &move2) {
                println!("{:?}", e );
                continue;
            }
            if let Err(e) = game.board.play_move(&move1, &move2) {
                println!("{:?}", e);
                continue;
            }
            println!("{}", game.board);
        }

    }
}
        