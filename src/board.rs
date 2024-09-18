use core::fmt;
use std::slice::Iter;
use self::RowLetter::*;

use crate::pieces::*;

#[derive(Debug)]
pub enum RowLetter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl RowLetter {
   pub fn eval(&self) -> usize {
    match self {
         RowLetter::A => 0,
         RowLetter::B => 1,
         RowLetter::C => 2,
         RowLetter::D => 3,
         RowLetter::E => 4,
         RowLetter::F => 5,
         RowLetter::G => 6,
         RowLetter::H => 7
    }
   } 

   pub fn iterator() -> Iter<'static, RowLetter> {
    static LETTERS: [RowLetter; 8] = [A, B, C, D, E, F, G, H];
    LETTERS.iter()
   } 

   pub fn convert_to(letter: char) -> Result<RowLetter, &'static str> {
    match letter {
        'a' => Ok(RowLetter::A),
        'b' => Ok(RowLetter::B),
        'c' => Ok(RowLetter::C),
        'd' => Ok(RowLetter::D),
        'e' => Ok(RowLetter::E),
        'f' => Ok(RowLetter::F),
        'g' => Ok(RowLetter::G),
        'h' => Ok(RowLetter::H),
        _ => Err("Not a valid col")
    }
   }
}

impl fmt::Display for RowLetter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Coordinates {
    pub letter: RowLetter,
    pub number: usize
}

#[derive(Clone, Copy)]
pub enum Square {
    Empty,
    Full(Piece)
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "[  ]"),
            Self::Full(piece) => write!(f, "[{}{}]",piece.color, piece.kind)
        }
    }
}

pub struct Row {
    squares: [Square; 8]
}

impl fmt::Display for Row {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        for square in self.squares.iter() {
            print!("{}", square);
        }
        Ok(())
    }
}

impl Row {
    pub fn default() -> Self {
        Row {
            squares: [Square::Empty; 8]
        }
    }

    pub fn pawn_row(piece_color: PieceColor) -> Self {
        Row {
            squares: [Square::Full(Piece{
                color: piece_color,
                kind: PieceKind::Pawn
            }); 8]
        }
    }

    pub fn default_back_row(piece_color: PieceColor) -> Self {
        Row {
            squares: [
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Rook
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Knight
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Bishop
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Queen
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::King
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Bishop
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Knight
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Rook
                }),
            ]
        }
    }
}

pub struct Board {
    rows: [Row; 8]
}

impl Board {
    pub fn default() -> Self {
        Board {
            rows: [
                Row::default_back_row(PieceColor::Black),
                Row::pawn_row(PieceColor::Black),
                Row::default(),
                Row::default(),
                Row::default(),
                Row::default(),
                Row::pawn_row(PieceColor::White),
                Row::default_back_row(PieceColor::White)
            ]
        }
    }
    
    fn convert_row_usize(size: usize) -> usize {
        7 - (size - 1)
    }

    fn retreive_square(&mut self, coords: &Coordinates) -> &mut Square {
        &mut self.rows[Self::convert_row_usize(coords.number)].squares[coords.letter.eval()]
    }

    fn set_square(&mut self, coords: &Coordinates, square: Square) {
        self.rows[Self::convert_row_usize(coords.number)].squares[coords.letter.eval()] = square;
    }

    pub fn move_piece(&mut self, from: Coordinates, to: Coordinates) {
        let from_square = self.retreive_square(&from);
        let mut replacement_square = Square::Empty;
        match from_square {
            Square::Full(piece) => {
                //to_square = Square::Full(piece.clone());
                replacement_square = Square::Full(piece.clone());
                ()}, //
            _ => ()
        }
        self.set_square(&from, Square::Empty);
        self.set_square(&to, replacement_square);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 8;
        for row in self.rows.iter() {
            print!("[{} ]", i);
            i -= 1;
            print!("{}\n", row);
        }
        print!("    ");
        for letter in RowLetter::iterator() {
            print!("[{} ]", letter)
        }
        Ok(())
    }
}
