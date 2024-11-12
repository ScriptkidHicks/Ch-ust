use std::str;

use crate::{board::{Board, Row, Square}, pieces::{Piece, PieceColor, PieceKind}};

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