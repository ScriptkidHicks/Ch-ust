use board::{Board, Coordinates};
use interface::parse_square;

mod board;
mod pieces;
mod interface;
mod rules;
fn main() {
    let mut default_board = Board::default();

    println!("This is a board\n{}", default_board);
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::G, number: 1}, &Coordinates{letter: board::ColumnLetter::F, number: 3});
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::G, number: 8}, &Coordinates{letter: board::ColumnLetter::F, number: 6});

    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::F, number: 3}, &Coordinates{letter: board::ColumnLetter::G, number: 5});
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::F, number: 6}, &Coordinates{letter: board::ColumnLetter::G, number: 4});

    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::G, number: 5}, &Coordinates{letter: board::ColumnLetter::H, number: 7});
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::G, number: 4}, &Coordinates{letter: board::ColumnLetter::H, number: 2});

    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::H, number: 7}, &Coordinates{letter: board::ColumnLetter::F, number: 6});
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::H, number: 2}, &Coordinates{letter: board::ColumnLetter::F, number: 3});
    
    
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::F, number: 6}, &Coordinates{letter: board::ColumnLetter::E, number: 4});
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::H, number: 8}, &Coordinates{letter: board::ColumnLetter::A, number: 8});

    println!("the board after moves\n{}", default_board);
}
