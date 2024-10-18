use board::{Board, ColumnLetter, Coordinates};

mod board;
mod pieces;
mod interface;
mod rules;
fn main() {
    let mut default_board = Board::default();

    println!("This is a board\n{}", default_board);
    let knight_coords= Coordinates{ letter: ColumnLetter::B, number: 1};
    default_board.show_me_legal_squares(&knight_coords); 
    let coord_a= Coordinates{letter: ColumnLetter::A, number: 2};
    let coord_b = Coordinates{letter: ColumnLetter::A, number: 3};
    default_board.show_me_legal_squares(&coord_a);
    default_board.move_piece(&coord_a, &coord_b);
    println!("this is the board\n{}", default_board);
    default_board.show_me_legal_squares(&knight_coords);

}
