use crate::board::*;

fn parse_char_to_int(row_number: char) -> Result<isize, &'static str> {
    match row_number {
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        _ => Err("invalid row")
    }
}

pub fn parse_square(input_string: &str) -> Result<Coordinates, &'static str> {
    let col: ColumnLetter;
    let row: isize;
    if input_string.len() != 2 {
        Err("incorrect length")
    } else {
        match ColumnLetter::convert_to(input_string.chars().nth(0).unwrap()) {
            Ok(col_letter) => {col = col_letter},
            Err(col_error) =>{
                    println!("couldn't convert column letter: {}", input_string.chars().nth(0).unwrap());
                 return Err(col_error)}
        }

        match parse_char_to_int(input_string.chars().nth(1).unwrap()) {
            Ok(row_number) => row = row_number,
            Err(row_error) => {
                println!("couldn't parse int: {}", input_string.chars().nth(1).unwrap());
                return Err(row_error)}
        }
        Ok(Coordinates {
            letter: col,
            number: row
        })
    }
}

