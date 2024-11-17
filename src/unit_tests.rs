use std::{default, str};

use crate::{board::{Board, ColumnLetter, Coordinates, MoveResult, Row, SideInformation, Square}, pieces::{Piece, PieceColor, PieceKind}};

#[test]
fn test_default_row_empty() {
    let default_row = Row::default();
    let manually_constructed_default = Row::new( [Square::Empty; 8]);

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
    let manually_constructed_default = Row::new([Square::Full(Piece {color: PieceColor::Black, kind: PieceKind::Pawn}); 8]);

    assert_eq!(default_row, manually_constructed_default);
}

#[test]
fn test_default_pawn_row_white() {
    let default_row = Row::pawn_row(PieceColor::White);
    let manually_constructed_default = Row::new([Square::Full(Piece {color: PieceColor::White, kind: PieceKind::Pawn}); 8]);

    assert_eq!(default_row, manually_constructed_default);
}

#[test]
fn test_default_back_row_white() {
    let default_row = Row::default_back_row(PieceColor::White);
    let manually_constructed_default = Row::new([
        Square::Full(Piece {color: PieceColor::White, kind: PieceKind::Rook}), 
        Square::Full(Piece {color: PieceColor::White, kind: PieceKind::Knight}), 
        Square::Full(Piece {color: PieceColor::White, kind: PieceKind::Bishop}), 
        Square::Full(Piece {color: PieceColor::White, kind: PieceKind::Queen}), 
        Square::Full(Piece {color: PieceColor::White, kind: PieceKind::King}), 
        Square::Full(Piece {color: PieceColor::White, kind: PieceKind::Bishop}), 
        Square::Full(Piece {color: PieceColor::White, kind: PieceKind::Knight}), 
        Square::Full(Piece {color: PieceColor::White, kind: PieceKind::Rook})]);

    assert_eq!(default_row, manually_constructed_default);
}

#[test]
fn test_default_back_row_black() {
    let default_row = Row::default_back_row(PieceColor::Black);
    let manually_constructed_default = Row::new([
        Square::Full(Piece {color: PieceColor::Black, kind: PieceKind::Rook}), 
        Square::Full(Piece {color: PieceColor::Black, kind: PieceKind::Knight}), 
        Square::Full(Piece {color: PieceColor::Black, kind: PieceKind::Bishop}), 
        Square::Full(Piece {color: PieceColor::Black, kind: PieceKind::Queen}), 
        Square::Full(Piece {color: PieceColor::Black, kind: PieceKind::King}), 
        Square::Full(Piece {color: PieceColor::Black, kind: PieceKind::Bishop}), 
        Square::Full(Piece {color: PieceColor::Black, kind: PieceKind::Knight}), 
        Square::Full(Piece {color: PieceColor::Black, kind: PieceKind::Rook})]);

    assert_eq!(default_row, manually_constructed_default);
}

#[test]
fn test_pawn_single_step() {
    let mut default_board: Board = Board::default();

    let mut jumped_to_row = Row::default();
    jumped_to_row.set_square(0, Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Pawn}));
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
            Row::default_back_row(PieceColor::White)

        ],
        PieceColor::Black,
        SideInformation::default(PieceColor::White),
        SideInformation::default(PieceColor::Black)
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
    jumped_to_row.set_square(0, Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Pawn}));
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
            Row::default_back_row(PieceColor::White)

        ],
        PieceColor::Black,
        SideInformation::default(PieceColor::White),
        SideInformation::default(PieceColor::Black)
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
}