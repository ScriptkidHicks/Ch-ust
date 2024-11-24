use std::{convert, f32::RADIX, fs, path::Path};

use crate::{
    board::{Board, ColumnLetter, Coordinates, Row, SideInformation, Square},
    interface::parse_char_to_isize,
    pieces::{Piece, PieceColor, PieceKind},
};

pub fn ingest_fen_file(file_path: &str) -> Option<Board> {
    let mut board_result: Option<Board> = None;

    if path_exists(file_path) {
        match digest_filepath_to_string(file_path) {
            Some(string_result) => {
                match digest_string_to_board(string_result) {
                    Some(board) => {
                        board_result = Some(board);
                    }
                    None => {
                        println!("Oops! Looks like the contents of that file couldn't be parsed correctly.");
                    }
                }
            }
            None => {
                println!(
                    "Oops! looks like we couldn't get a string from {}",
                    file_path
                );
            }
        }
    } else {
        println!("It looks like the file {} does exist!", file_path);
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

pub fn digest_string_to_board(file_contents: String) -> Option<Board> {
    let mut accum_rows: Vec<Row> = Vec::new();
    let turn_color: PieceColor;
    let white_castle_kingside: bool;
    let white_castle_queenside: bool;
    let black_castle_kingside: bool;
    let black_castle_queenside: bool;
    let mut opt_passant_square: Option<Coordinates> = None;
    let half_turns: u32;
    let full_turns: u32;

    let string_parts = file_contents.split(' ').collect::<Vec<&str>>();

    //There must be exactly six parts.
    if (string_parts.len() != 6) {
        return None;
    }

    // the 0th string will always be the rows
    digest_board_string_into_rows(string_parts.get(0), &mut accum_rows);

    //the 1s position string will always be whose turn it is
    match parse_char_to_turn_color(string_parts.get(1)) {
        Some(turn) => {
            turn_color = turn;
        }
        None => {
            return None;
        }
    }

    //the 2nd position string will always be castling rights
    match string_parts.get(2) {
        Some(reference_string) => {
            (
                white_castle_kingside,
                white_castle_queenside,
                black_castle_kingside,
                black_castle_queenside,
            ) = parse_string_to_castling_rights(reference_string);
        }
        None => {
            return None;
        }
    }

    //deriving the optional passant square
    match string_parts.get(3) {
        Some(passant_string) => {
            opt_passant_square = parse_string_into_passant_square(passant_string);
        }
        None => {
            return None;
        }
    }

    //get the half turns
    match string_parts.get(4) {
        Some(half_turn_string) => match half_turn_string.parse::<u32>() {
            Ok(half_turns_found) => {
                half_turns = half_turns_found;
            }
            Err(_) => {
                println!("Could not correctly parse half turns.");
                return None;
            }
        },
        None => {
            return None;
        }
    }

    //get the full turns
    match string_parts.get(5) {
        Some(full_turn_string) => match full_turn_string.parse::<u32>() {
            Ok(full_turns_found) => {
                full_turns = full_turns_found;
            }
            Err(_) => {
                println!("Could not correctly parse half turns.");
                return None;
            }
        },
        None => {
            return None;
        }
    }

    //if we have reached here we can return safely
    let rows_as_array: [Row; 8] = accum_rows.try_into().expect(
        "Something appears to have incorrectly converted in moving accumulated rows to an array",
    );

    let mut white_information = SideInformation::default(PieceColor::White);
    white_information.set_castling_rights(white_castle_kingside, white_castle_queenside);

    let mut black_information = SideInformation::default(PieceColor::Black);
    black_information.set_castling_rights(black_castle_kingside, black_castle_queenside);

    let mut return_board = Board::new(
        rows_as_array,
        turn_color,
        opt_passant_square,
        white_information,
        black_information,
        half_turns,
        full_turns,
    );

    //we need to make sure that each side displays the correct number of taken pieces.
    return_board.adjust_taken_pieces();

    Some(return_board)
}

pub fn digest_board_string_into_rows(opt_row_text: Option<&&str>, row_collection: &mut Vec<Row>) {
    match opt_row_text {
        Some(row_text) => {
            let row_strings = row_text.split("/").collect::<Vec<&str>>();
            if (row_strings.len() != 8) {
                return; // return early. You should have exactly 8 of these suckers.
            }

            for row_string in row_strings.iter() {
                match digest_row_string_to_row(row_string) {
                    Some(row) => {
                        row_collection.push(row);
                    }
                    None => {
                        // if you failed to add the row, return early.
                        return;
                    }
                }
            }
        }
        None => {
            //dawg what?
        }
    }
}

pub fn digest_row_string_to_row(row_string: &&str) -> Option<Row> {
    let mut squares: Vec<Square> = Vec::new();

    let mut opt_row: Option<Row> = None;

    for character in row_string.chars() {
        if character.is_numeric() {
            //this is in base 10. If you want to go look up what a radix is, I recommend
            //https://doc.rust-lang.org/std/primitive.char.html#method.to_digit
            match character.to_digit(10) {
                Some(digit) => {
                    for _ in 0..digit {
                        //on christ, I am going to go put a pull request into rust to fix this bullshit.
                        squares.push(Square::Empty);
                    }
                }
                None => {
                    //break early. You passed an illegal digit I guess? How?
                    println!("the character {} was found while trying to parse a fen file for a digit. Sorry.", character);
                    return None;
                }
            }
        } else if character.is_alphabetic() {
            match parse_char_to_piece(character) {
                Some(piece) => {
                    squares.push(Square::Full(piece));
                }
                None => {
                    return None;
                }
            }
        }
    }

    if (squares.len() == 8) {
        let squares_array: [Square; 8] = squares.try_into().expect(
            "Something appears to have converted incorrectly in converting this row to squares",
        );

        opt_row = Some(Row::new(squares_array));
    }

    opt_row
}

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

pub fn parse_char_to_turn_color(opt_turn_char: Option<&&str>) -> Option<PieceColor> {
    let mut opt_piece_color: Option<PieceColor> = None;
    match opt_turn_char {
        Some(turn_string) => {
            if (turn_string.len() != 1) {
                return None;
            }

            for char in turn_string.chars() {
                match char.to_ascii_lowercase() {
                    'b' => opt_piece_color = Some(PieceColor::Black),
                    'w' => opt_piece_color = Some(PieceColor::White),
                    _ => {}
                }
            }
        }
        None => {}
    }

    opt_piece_color
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

pub fn parse_string_to_castling_rights(input_str: &&str) -> (bool, bool, bool, bool) {
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

pub fn parse_string_into_passant_square(input_str: &&str) -> Option<Coordinates> {
    let mut opt_coordinates: Option<Coordinates> = None;
    let mut opt_col_letter: Option<ColumnLetter> = None;
    let mut opt_row_number: Option<isize> = None;

    if input_str.len() > 2 {
        println!("Encountered an error while attempting to parse the viable en-passant square. String was the wrong size.");
        return None;
    }

    for char in input_str.chars() {
        if char.is_alphabetic() {
            match ColumnLetter::convert_to(char) {
                Ok(column_letter) => {
                    opt_col_letter = Some(column_letter);
                }
                Err(_) => {
                    return None;
                }
            }
        } else if char.is_numeric() {
            //see earlier where we use radix for an explanation
            match parse_char_to_isize(char) {
                Ok(size_result) => {
                    opt_row_number = Some(size_result);
                }
                Err(_) => {
                    println!("Digit was outside the bounds fo the board in parse passant string");
                    return None;
                }
            }
        } else if char == '-' {
            return None;
        }
    }

    match opt_col_letter {
        Some(col_let) => match opt_row_number {
            Some(row_num) => {
                opt_coordinates = Some(Coordinates {
                    letter: col_let,
                    number: row_num,
                });
            }
            None => {}
        },
        None => {}
    }

    opt_coordinates
}
