use board::{Board, Coordinates};
use interface::parse_square;

mod board;
mod pieces;
mod interface;
mod rules;
fn main() {
    let mut default_board = Board::default();

    println!("This is a board\n{}", default_board);
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::B, number: 1}, &Coordinates{letter: board::ColumnLetter::C, number: 3});

    println!(" after move one: {}", default_board);

    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::C, number: 3}, &Coordinates{letter: board::ColumnLetter::B, number: 5});

    println!(" after move two: {}", default_board);

    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::B, number: 5}, &Coordinates{letter: board::ColumnLetter::C, number: 7});

    println!(" after move three: {}", default_board);

    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::C, number: 7}, &Coordinates{letter: board::ColumnLetter::D, number: 5});

    println!(" after move three: {}", default_board);

    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::D, number: 5}, &Coordinates{letter: board::ColumnLetter::E, number: 7});

    println!(" after move three: {}", default_board);

    
}
