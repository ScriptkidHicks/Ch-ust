use board::{Board, Coordinates};
use interface::parse_square;

mod board;
mod pieces;
mod interface;
mod rules;
fn main() {
    let mut default_board = Board::default();

    println!("This is a board\n{}", default_board);

    println!("lets make a move from e2 to e3");

    let from: Coordinates;
    let to: Coordinates;

    match parse_square("e2") {
        Ok(from_coords) => {from = from_coords;
            match parse_square("z9") {
                Ok(to_coords) => {
                    to = to_coords;
                    default_board.move_piece(from, to);
                    println!("this is now the board\n{}", default_board);
                },
                Err(to_message) => println!("we had the error: {}", to_message)
            }
        },
        Err(from_message) => println!("we had the error: {}", from_message)
    }

    
}
