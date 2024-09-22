use core::fmt;
use std::{iter::TakeWhile, slice::Iter};
use self::ColumnLetter::*;

use crate::{pieces::*, rules::parse_move_legality};

pub fn usize_difference(a: usize, b: usize) -> usize {
    if a > b { 
        a - b
    } else {
        b - a
    }
}

#[derive(Debug, PartialEq)]
pub enum ColumnLetter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl ColumnLetter {
   pub fn eval(&self) -> usize {
    match self {
         ColumnLetter::A => 0,
         ColumnLetter::B => 1,
         ColumnLetter::C => 2,
         ColumnLetter::D => 3,
         ColumnLetter::E => 4,
         ColumnLetter::F => 5,
         ColumnLetter::G => 6,
         ColumnLetter::H => 7
    }
   } 

   pub fn iterator() -> Iter<'static, ColumnLetter> {
    static LETTERS: [ColumnLetter; 8] = [A, B, C, D, E, F, G, H];
    LETTERS.iter()
   } 

   pub fn convert_to(letter: char) -> Result<ColumnLetter, &'static str> {
    match letter {
        'a' => Ok(ColumnLetter::A),
        'b' => Ok(ColumnLetter::B),
        'c' => Ok(ColumnLetter::C),
        'd' => Ok(ColumnLetter::D),
        'e' => Ok(ColumnLetter::E),
        'f' => Ok(ColumnLetter::F),
        'g' => Ok(ColumnLetter::G),
        'h' => Ok(ColumnLetter::H),
        _ => Err("Not a valid col")
    }
   }
}

impl fmt::Display for ColumnLetter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum MoveDirection {
    Verticle,
    Lateral,
    Diagonal,
    JHook,
    NoMove,
    IllegalMove
}

pub struct SquareToSquareInformation {
    pub move_direction: MoveDirection,
    pub distance: usize

}
pub struct Coordinates {
    pub letter: ColumnLetter,
    pub number: usize
}

pub fn measure_distance(from: &Coordinates, to: &Coordinates) -> SquareToSquareInformation {
    let garnered_move_direction: MoveDirection;

    let lat_distance = usize_difference(from.letter.eval(), to.letter.eval()); 

    let vert_distance = usize_difference(from.number, to.number);

    if from.letter == to.letter {
        if from.number == to.number {
            garnered_move_direction = MoveDirection::NoMove;
        } else {
            garnered_move_direction = MoveDirection::Verticle;
        }
    } else if from.number == to.number {
        garnered_move_direction = MoveDirection::Lateral;
    } else {
        if lat_distance == vert_distance {
            garnered_move_direction = MoveDirection::Diagonal;
        } else if (lat_distance == 1 && vert_distance == 2) || (vert_distance == 1 && lat_distance == 2) {
            garnered_move_direction = MoveDirection::JHook;
        } else {
            garnered_move_direction = MoveDirection::IllegalMove;
        }
    }

    SquareToSquareInformation {
        move_direction: garnered_move_direction,
        distance: lat_distance + vert_distance
    }
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

struct SideInformation {
    taken_pieces: Vec<PieceKind>,
    can_castle_kingside: bool,
    can_castle_queenside: bool
}

impl SideInformation {
    pub fn default () -> Self {
        SideInformation {
            taken_pieces: Vec::new(),
            can_castle_kingside: true,
            can_castle_queenside: false
        }
    }

    pub fn add_taken_piece (&mut self, piece_kind: PieceKind) {
        self.taken_pieces.push(piece_kind);
    }
}

pub struct Board {
    rows: [Row; 8],
    turn: PieceColor,
    white_side_information: SideInformation,
    black_side_information: SideInformation
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
            ],
            turn: PieceColor::White,
            white_side_information: SideInformation::default(),
            black_side_information: SideInformation::default()
        }
    }
    
    fn convert_row_usize(size: usize) -> usize {
        7 - (size - 1)
    }

    pub fn retreive_square(&self, coords: &Coordinates) -> &Square {
        &self.rows[Self::convert_row_usize(coords.number)].squares[coords.letter.eval()]
    }

    fn set_square(&mut self, coords: &Coordinates, square: Square) {
        self.rows[Self::convert_row_usize(coords.number)].squares[coords.letter.eval()] = square;
    }

    pub fn move_piece(&mut self, from: &Coordinates, to: &Coordinates) {
        let from_square = self.retreive_square(&from);
        let to_square = self.retreive_square(&to);
        let mut move_legal = false;
        let mut taking_piece = false;
        let mut target_piece_kind = PieceKind::Pawn;
        let mut target_piece_color = PieceColor::Black;
        let mut replacement_square = Square::Empty;
        match from_square {
            Square::Full(piece) => {
                (move_legal, taking_piece, target_piece_color, target_piece_kind) = parse_move_legality(piece.kind, from, to, self);
                //to_square = Square::Full(piece.clone());
                replacement_square = Square::Full(piece.clone());
                ()}, //
            _ => ()
        }
        if move_legal {
            self.set_square(&from, Square::Empty);
            self.set_square(&to, replacement_square);
            if (taking_piece) {
                self.add_piece_to_kills(target_piece_kind, target_piece_color);
            }
        }
    }

    pub fn add_piece_to_kills(&mut self, piece_kind: PieceKind, piece_color: PieceColor) {
        match piece_color {
            PieceColor::Black => self.white_side_information.add_taken_piece(piece_kind),
            PieceColor::White => self.black_side_information.add_taken_piece(piece_kind),
        }
    }

    pub fn show_taken_pieces(&self, color: PieceColor) {
        print!("\n< ");
        match color {
            PieceColor::Black => {
                for piece in self.black_side_information.taken_pieces.iter() {
                    print!("{} ", piece);
                }
            },
            PieceColor::White => {
                for piece in self.white_side_information.taken_pieces.iter() {
                    print!("{} ", piece);
                }
            }
        }
        println!(">");
    }
}

impl fmt::Display for Board {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        self.show_taken_pieces(PieceColor::Black);
        let mut i = 8;
        for row in self.rows.iter() {
            print!("[{} ]", i);
            i -= 1;
            print!("{}\n", row);
        }
        print!("    ");
        for letter in ColumnLetter::iterator() {
            print!("[{} ]", letter)
        }
        self.show_taken_pieces(PieceColor::White);
        Ok(())
    }
}
