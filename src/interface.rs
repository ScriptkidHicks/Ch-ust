use crate::board::*;
use crate::pieces::*;

fn parse_char_to_int(row_number: char) -> Result<usize, &'static str> {
    match row_number {
        '1' => Ok(1),
        _ => Err("input is not a valid row number")
    }
}

pub fn parse_square(input_string: &str) -> Result<Coordinates, &'static str> {
    let mut col: RowLetter;
    let mut row: usize;
    if (input_string.len() != 2){
        Err("incorrect length")
    } else {
        match RowLetter::convert_to(input_string.chars().nth(0).unwrap()) {
            Ok(col_letter) => {col = col_letter},
            Err(col_error) => return Err(col_error)
        }

        match parse_char_to_int(input_string.chars().nth(1).unwrap()) {
            Ok(row_number) => row = row_number,
            Err(row_error) => return Err(row_error)
        }

        print!("first char: {}", input_string.chars().nth(0).unwrap());
        Err("incorrect length")
    }
}

