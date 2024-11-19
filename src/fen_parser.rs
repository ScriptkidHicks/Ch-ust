use std::{fs, path::Path};

use crate::{
    board::{Board, Row},
    pieces::{Piece, PieceColor, PieceKind},
};

pub fn ingest_fen_file(file_path: &str) -> Option<Board> {
    let mut board_result: Option<Board> = None;

    if path_exists(file_path) {
        match digest_filepath_to_string(file_path) {
            Some(string_result) => {
                match parse_string_to_board(string_result) {
                    Some(board) => {
                        board_result = Some(board);
                    }
                    None => {
                        println!("Oops! Looks like the contents of that file couldn't be parsed correctly.");
                    }
                }
            }
            None => {
                println!("Oops! looks like we couldn't get a string from that file");
            }
        }
    }

    board_result
}

pub fn path_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

pub fn digest_filepath_to_string(file_path: &str) -> Option<String> {
    match fs::read_to_string(file_path) {
        Ok(string_value) => Some(string_value),
        Err(_) => None,
    }
}

pub fn parse_string_to_board(file_contents: String) -> Option<Board> {
    let mut opt_board_return: Option<Board> = None;
    let mut accum_rows: Vec<Row> = Vec::new();

    let string_parts = file_contents.split(' ');

    for character in file_contents.chars() {}

    if (accum_rows.len() == 8) {}

    opt_board_return
}

pub fn fen_parsing_switchboard(rows: &mut Vec<Row>) {}

pub fn parse_char_to_piecekind(input_char: char) -> Option<PieceKind> {
    match input_char {
        'r' => Some(PieceKind::Rook),
        'p' => Some(PieceKind::Pawn),
        'n' => Some(PieceKind::Knight),
        'k' => Some(PieceKind::King),
        'q' => Some(PieceKind::Queen),
        'b' => Some(PieceKind::Bishop),
        _ => None,
    }
}

pub fn parse_char_to_turn_color(input_char: char) -> Option<PieceColor> {
    match input_char.to_ascii_lowercase() {
        'b' => Some(PieceColor::Black),
        'w' => Some(PieceColor::White),
        _ => None,
    }
}

pub fn parse_char_to_piece(input_char: char) -> Option<Piece> {
    let lowercased_input = input_char.to_ascii_lowercase();
    let piece_color: PieceColor = if input_char == lowercased_input {
        PieceColor::Black
    } else {
        PieceColor::White
    };

    match parse_char_to_piecekind(lowercased_input) {
        Some(piece_kind) => Some(Piece {
            kind: piece_kind,
            color: piece_color,
        }),
        None => None,
    }
}

pub fn parse_string_to_castling_rights(input_str: &str) -> (bool, bool, bool, bool) {
    let mut black_kingside = false;
    let mut black_queenside = false;
    let mut white_kingside = false;
    let mut white_queenside = false;

    for character in input_str.chars() {
        match character {
            'K' => white_kingside = true,
            'Q' => white_queenside = true,
            'k' => black_kingside = true,
            'q' => black_queenside = true,
            _ => {
                //Should I panic here?
            }
        }
    }

    (
        white_kingside,
        white_queenside,
        black_kingside,
        black_queenside,
    )
}
