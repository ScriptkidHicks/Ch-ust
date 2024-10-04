use board::{Board, Coordinates};

mod board;
mod pieces;
mod interface;
mod rules;
fn main() {
    let mut default_board = Board::default();

    println!("This is a board\n{}", default_board);
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::E, number: 2}, &Coordinates{letter: board::ColumnLetter::E, number: 4});
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::E, number: 7}, &Coordinates{letter: board::ColumnLetter::E, number: 5});
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::G, number: 1}, &Coordinates{letter: board::ColumnLetter::F, number: 3});
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::G, number: 8}, &Coordinates{letter: board::ColumnLetter::F, number: 6});
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::F, number: 1}, &Coordinates{letter: board::ColumnLetter::D, number: 3});
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::F, number: 8}, &Coordinates{letter: board::ColumnLetter::D, number: 6});

    println!("the board before castling\n{}", default_board);

    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::E, number: 1}, &Coordinates{letter: board::ColumnLetter::G, number: 1});
    default_board.move_piece(&Coordinates{letter: board::ColumnLetter::E, number: 8}, &Coordinates{letter: board::ColumnLetter::G, number: 8});

    println!("the board after castling\n{}", default_board);
}
