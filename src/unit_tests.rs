use crate::{
    board::{Board, ColumnLetter, Coordinates, MoveResult, Row, SideInformation, Square},
    fen_parser::ingest_fen_file,
    pieces::{Piece, PieceColor, PieceKind},
};

#[test]
fn test_cloning_board_works() {
    let default_board = Board::default();
    let copied_board = default_board.clone();

    assert!(default_board == copied_board);
}

#[test]
fn test_default_row_empty() {
    let default_row = Row::default();
    let manually_constructed_default = Row::new([Square::Empty; 8]);

    assert_eq!(default_row, manually_constructed_default);
}

#[test]
fn test_default_row_empty_assume_failure() {
    let default_row = Row::default();
    let manually_constructed_default = Row::default_back_row(PieceColor::Black);

    assert_ne!(default_row, manually_constructed_default);
}

#[test]
fn test_default_pawn_row_black() {
    let default_row = Row::pawn_row(PieceColor::Black);
    let manually_constructed_default = Row::new(
        [Square::Full(Piece {
            color: PieceColor::Black,
            kind: PieceKind::Pawn,
        }); 8],
    );

    assert_eq!(default_row, manually_constructed_default);
}

#[test]
fn test_default_pawn_row_white() {
    let default_row = Row::pawn_row(PieceColor::White);
    let manually_constructed_default = Row::new(
        [Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::Pawn,
        }); 8],
    );

    assert_eq!(default_row, manually_constructed_default);
}

#[test]
fn test_default_back_row_white() {
    let default_row = Row::default_back_row(PieceColor::White);
    let manually_constructed_default = Row::new([
        Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::Rook,
        }),
        Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::Knight,
        }),
        Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::Bishop,
        }),
        Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::Queen,
        }),
        Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::King,
        }),
        Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::Bishop,
        }),
        Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::Knight,
        }),
        Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::Rook,
        }),
    ]);

    assert_eq!(default_row, manually_constructed_default);
}

#[test]
fn test_default_back_row_black() {
    let default_row = Row::default_back_row(PieceColor::Black);
    let manually_constructed_default = Row::new([
        Square::Full(Piece {
            color: PieceColor::Black,
            kind: PieceKind::Rook,
        }),
        Square::Full(Piece {
            color: PieceColor::Black,
            kind: PieceKind::Knight,
        }),
        Square::Full(Piece {
            color: PieceColor::Black,
            kind: PieceKind::Bishop,
        }),
        Square::Full(Piece {
            color: PieceColor::Black,
            kind: PieceKind::Queen,
        }),
        Square::Full(Piece {
            color: PieceColor::Black,
            kind: PieceKind::King,
        }),
        Square::Full(Piece {
            color: PieceColor::Black,
            kind: PieceKind::Bishop,
        }),
        Square::Full(Piece {
            color: PieceColor::Black,
            kind: PieceKind::Knight,
        }),
        Square::Full(Piece {
            color: PieceColor::Black,
            kind: PieceKind::Rook,
        }),
    ]);

    assert_eq!(default_row, manually_constructed_default);
}

#[test]
fn test_pawn_single_step() {
    let mut default_board: Board = Board::default();

    let mut jumped_to_row = Row::default();
    jumped_to_row.set_square(
        0,
        Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::Pawn,
        }),
    );
    let mut altered_pawn_row = Row::pawn_row(PieceColor::White);
    altered_pawn_row.set_square(0, Square::Empty);

    let stepped_board: Board = Board::new(
        [
            Row::default_back_row(PieceColor::Black),
            Row::pawn_row(PieceColor::Black),
            Row::default(),
            Row::default(),
            Row::default(),
            jumped_to_row,
            altered_pawn_row,
            Row::default_back_row(PieceColor::White),
        ],
        PieceColor::Black,
        None,
        SideInformation::default(PieceColor::White),
        SideInformation::default(PieceColor::Black),
        0,
        1,
    );

    let pawn_from_coords = Coordinates::new(ColumnLetter::A, 2);
    let pawn_to_coords = Coordinates::new(ColumnLetter::A, 3);

    default_board.move_piece(&pawn_from_coords, &pawn_to_coords);

    let boards_equal = default_board == stepped_board;
    assert!(boards_equal);
}

#[test]
pub fn test_pawn_double_step() {
    let mut default_board: Board = Board::default();

    let mut jumped_to_row = Row::default();
    jumped_to_row.set_square(
        0,
        Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::Pawn,
        }),
    );
    let mut altered_pawn_row = Row::pawn_row(PieceColor::White);
    altered_pawn_row.set_square(0, Square::Empty);

    let stepped_board: Board = Board::new(
        [
            Row::default_back_row(PieceColor::Black),
            Row::pawn_row(PieceColor::Black),
            Row::default(),
            Row::default(),
            jumped_to_row,
            Row::default(),
            altered_pawn_row,
            Row::default_back_row(PieceColor::White),
        ],
        PieceColor::Black,
        Some(Coordinates {
            letter: ColumnLetter::A,
            number: 3,
        }),
        SideInformation::default(PieceColor::White),
        SideInformation::default(PieceColor::Black),
        0,
        1,
    );

    let pawn_from_coords = Coordinates::new(ColumnLetter::A, 2);
    let pawn_to_coords = Coordinates::new(ColumnLetter::A, 4);

    default_board.move_piece(&pawn_from_coords, &pawn_to_coords);

    let boards_equal = default_board == stepped_board;

    assert!(boards_equal);
}

#[test]
pub fn test_wrong_side_turn() {
    let mut default_board = Board::default();

    let pawn_from = Coordinates::new(ColumnLetter::A, 7);
    let pawn_to = Coordinates::new(ColumnLetter::A, 6);

    let move_result = default_board.move_piece(&pawn_from, &pawn_to);

    assert_eq!(move_result, MoveResult::WrongTurn);
}

#[test]
pub fn test_en_passant_should_pass() {
    let mut altered_white_advance_row = Row::default();
    altered_white_advance_row.set_square(
        3,
        Square::Full(Piece {
            color: PieceColor::White,
            kind: PieceKind::Pawn,
        }),
    );
    //this board is techinically an illegal state, but we don't care for the purpose of this test.
    let previous_turn_board = Board::new(
        [
            Row::default_back_row(PieceColor::Black),
            Row::pawn_row(PieceColor::Black),
            Row::default(),
            altered_white_advance_row,
            Row::default(),
            Row::default(),
            Row::pawn_row(PieceColor::White),
            Row::default_back_row(PieceColor::White),
        ],
        PieceColor::Black,
        None,
        SideInformation::default(PieceColor::White),
        SideInformation::default(PieceColor::Black),
        0,
        1,
    );

    let current_black_pawn_square_coords = Coordinates {
        letter: ColumnLetter::E,
        number: 7,
    };
    let black_pawn_jump_target_coords = Coordinates {
        letter: ColumnLetter::E,
        number: 5,
    };

    let mut current_turn_board = previous_turn_board.clone();

    let move_result = current_turn_board.move_piece(
        &current_black_pawn_square_coords,
        &black_pawn_jump_target_coords,
    );
    assert!(move_result == MoveResult::CompletedSafely);

    // we have moved the pawn forward. Now lets try to en passant
    let white_pawn_from_square = Coordinates {
        letter: ColumnLetter::D,
        number: 5,
    };
    let white_pawn_to_square = Coordinates {
        letter: ColumnLetter::E,
        number: 6,
    };
    let en_passant_move_result =
        current_turn_board.move_piece(&white_pawn_from_square, &white_pawn_to_square);

    assert!(en_passant_move_result == MoveResult::CompletedSafely);

    //now lets construct what the board should look like, and make sure not only that the move was legal, but the outcome was correct.
}

#[test]
fn test_basic_read() {
    let result = ingest_fen_file("./src/fenFiles/default_board.fen");
    match result {
        Some(board) => {
            let default_board = Board::default();
            assert!(board == default_board);
        }
        None => {
            assert!(false)
        }
    }
}
