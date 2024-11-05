use board::{Board, ColumnLetter, Coordinates};

mod board;
mod pieces;
mod interface;
mod rules;
fn main() {
    let mut default_board = Board::default();

    println!("This is a board\n{}", default_board);
    let king_coords= Coordinates{ letter: ColumnLetter::E, number: 1};
    default_board.show_me_legal_squares(&king_coords); 
    let knight_coords = Coordinates{ letter: ColumnLetter::B, number: 1};
    println!("lets take a look at knight squares");
    default_board.show_me_legal_squares(&knight_coords);
    let coord_a= Coordinates{letter: ColumnLetter::C, number: 2};
    let coord_b = Coordinates{letter: ColumnLetter::C, number: 3};
    let coord_c= Coordinates{letter: ColumnLetter::A, number: 7};
    let coord_d = Coordinates{letter: ColumnLetter::A, number: 6};
    let coord_e= Coordinates{letter: ColumnLetter::D, number: 2};
    let coord_f = Coordinates{letter: ColumnLetter::D, number: 4};
    default_board.move_piece(&coord_a, &coord_b);
    default_board.move_piece(&coord_c, &coord_d);
    default_board.move_piece(&coord_e, &coord_f);
    println!("this is the board\n{}", default_board);
    default_board.show_me_legal_squares(&king_coords);
    println!("the new target squares for knight");
    default_board.show_me_legal_squares(&knight_coords);
    println!("after");
    default_board.show_me_legal_squares(&Coordinates{letter: ColumnLetter::B, number: 2});

}
