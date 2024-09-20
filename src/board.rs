use core::fmt;
use std::slice::Iter;
use self::ColumnLetter::*;

use crate::pieces::*;


#[derive(Debug)]
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

impl PartialEq for ColumnLetter {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
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

pub struct SquareToSquareInformation {
    pub trueDiagonal: bool,
    pub trueVertOrLat: bool,
    pub trueJHook: bool,
    pub forwardMove: bool,
    pub distance: usize

}
pub struct Coordinates {
    pub letter: ColumnLetter,
    pub number: usize
}

impl Coordinates {
    pub fn measure_distance(&self, other: Coordinates) -> SquareToSquareInformation {
        let mut is_lat = false;
        let mut is_vert = false;
        let mut is_diag = false;
        let mut is_j = false;
        let mut distance: usize = 0;
        let mut lat_distance: usize = 0;
        let mut vert_distance: usize = 0;

        if (self.letter == other.letter) {
            is_vert = true;
        }

        if (self.number == other.number) {
            is_lat = true;
        }

        lat_distance = self.letter.eval() - other.letter.eval(); 
        vert_distance = self.number - other.number;

        if (!is_lat && !is_vert && vert_distance == lat_distance && lat_distance != 0) {
            is_diag = true;
        }

        if ((lat_distance == 1 && vert_distance == 2) || (vert_distance == 1 && lat_distance == 2)){
            is_j = true;
        }

        SquareToSquareInformation {
            trueDiagonal: is_diag,
            trueVertOrLat: is_lat || is_vert,
            trueJHook: is_j,
            forwardMove: true,
            distance: lat_distance + vert_distance
        }
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
        for letter in ColumnLetter::iterator() {
            print!("[{} ]", letter)
        }
        Ok(())
    }
}
