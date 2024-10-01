use core::fmt;
use std::{fmt::{write, Error}, ops::Range, path, slice::Iter, usize};
use self::ColumnLetter::*;

use crate::{pieces::*, rules::parse_move_legality};

pub fn usize_difference(a: usize, b: usize) -> usize {
    if a > b { 
        a - b
    } else {
        b - a
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

   pub fn construct_from_usize(size: usize) -> ColumnLetter {
    match size {
        0 => ColumnLetter::A,
        1 => ColumnLetter::B,
        2 => ColumnLetter::C,
        3 => ColumnLetter::D,
        4 => ColumnLetter::E,
        5 => ColumnLetter::F,
        6 => ColumnLetter::G,
        7 => ColumnLetter::H,
        _ => ColumnLetter::H
    }
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    JHook,
    NoMove,
    IllegalMove
}

impl fmt::Display for MoveDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Down => write!(f, "Down"),
            Self::Up => write!(f, "Up"),
            Self::Right => write!(f, "Right"),
            Self::Left => write!(f, "Left"),
            Self::JHook => write!(f, "JHook"),
            Self::DownLeft => write!(f, "Down Left"),
            Self::DownRight => write!(f, "Down Right"),
            Self::UpLeft => write!(f, "Up Left"),
            Self::UpRight => write!(f, "Up Right"),
            Self::IllegalMove => write!(f, "Illegal Move"),
            Self::NoMove => write!(f, "No Move")
        }
    }
}

pub struct SquareToSquareInformation {
    pub move_direction: MoveDirection,
    pub distance: usize
}

#[derive(Clone, Copy)]
pub struct Coordinates {
    pub letter: ColumnLetter,
    pub number: usize
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print!("{}{}", self.letter, self.number);
        Ok(())
    }
}

pub fn measure_distance(from: &Coordinates, to: &Coordinates) -> SquareToSquareInformation {
    let mut garnered_move_direction: MoveDirection = MoveDirection::NoMove;

    let lat_distance = usize_difference(from.letter.eval(), to.letter.eval()); 

    let vert_distance = usize_difference(from.number, to.number);

    if from.letter == to.letter {
        if from.number > to.number {
            garnered_move_direction = MoveDirection::Down;
        } else if from.number < to.number {
            garnered_move_direction = MoveDirection::Up;
        }
    } else if from.number == to.number {
        if from.letter.eval() > to.letter.eval() {
            garnered_move_direction = MoveDirection::Left;
        } else if from.letter.eval() < to.letter.eval() {
            garnered_move_direction = MoveDirection::Right;
        }
    } else {
        if lat_distance == vert_distance {
            //we know it's diagonal, and that movement has been made, so we need to determine direction
            if from.number > to.number {
                // we know the move is down.
                if from.letter.eval() > to.letter.eval() {
                    garnered_move_direction = MoveDirection::DownLeft;
                } else {
                    garnered_move_direction = MoveDirection::DownRight;
                }
            } else {
                // we know the move is up.
                if from.letter.eval() > to.letter.eval() {
                    garnered_move_direction = MoveDirection::UpLeft;
                } else {
                    garnered_move_direction = MoveDirection::UpRight;
                }
            }
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

#[derive(Clone)]
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

#[derive(Clone)]
struct SideInformation {
    taken_pieces: Vec<PieceKind>,
    can_castle_kingside: bool,
    can_castle_queenside: bool,
    current_king_square: Coordinates
}

impl SideInformation {
    pub fn default (king_color: PieceColor) -> Self {
        SideInformation {
            taken_pieces: Vec::new(),
            can_castle_kingside: true,
            can_castle_queenside: false,
            current_king_square: Coordinates {letter: ColumnLetter::E, number: match king_color {
                PieceColor::Black => 8,
                PieceColor::White => 1
            }}
        }
    }

    pub fn add_taken_piece (&mut self, piece_kind: PieceKind) {
        self.taken_pieces.push(piece_kind);
    }

    pub fn total_taken_pieces(&self) -> u32 {
        let mut total_value = 0;
        for piece in self.taken_pieces.iter() {
            total_value += piece.get_value()
        }
        total_value
    }

    pub fn update_king_location (&mut self, letter: ColumnLetter, number: usize) {
        self.current_king_square = Coordinates {letter: letter, number: number};
    }
}

pub enum MoveResult {
    MoveCompleted,
    WrongTurn,
    MoveIllegal,
    EmptySquare
}

#[derive(Clone)]
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
            white_side_information: SideInformation::default(PieceColor::White),
            black_side_information: SideInformation::default(PieceColor::Black)
        }
    }
    
    fn convert_row_usize(size: usize) -> usize {
        7 - (size - 1)
    }

    pub fn retreive_square(&self, coords: &Coordinates) -> &Square {
        &self.rows[Self::convert_row_usize(coords.number)].squares[coords.letter.eval()]
    }

    pub fn set_square(&mut self, coords: &Coordinates, square: Square) {
        self.rows[Self::convert_row_usize(coords.number)].squares[coords.letter.eval()] = square;
    }

    pub fn is_king_in_danger(&self, king_color: PieceColor) -> bool {

        let target_king_coordinates = match king_color {
            PieceColor::Black => self.black_side_information.current_king_square,
            PieceColor::White => self.white_side_information.current_king_square
        };

        for letter in ColumnLetter::iterator() {
            for number in 1..9 {
                let from_coords = Coordinates{letter: *letter, number: number};
                let gotten_square = self.retreive_square(&from_coords);
                match gotten_square {
                    &Square::Full(piece ) => {
                        // we only care about the ability of other pieces to tkae our king;
                        if piece.color != king_color {
                            //only pieces of the opposite color can threaten the king
                            if self.square_threatens_square(&from_coords, &target_king_coordinates) {
                                // oops, we found a square that threatens the king. Can't allow that!
                                return true;
                            }
                        }
                    },
                    &Square::Empty => ()
                }
            }
        }

        false
    }

    pub fn square_threatens_square(&self, from: &Coordinates, to: &Coordinates) -> bool {
        match self.retreive_square(from) {
            Square::Full(piece) => {
                let distance_information = measure_distance(from, to);
                let (legal, _, _, _): (bool, bool, PieceColor, PieceKind) = parse_move_legality(piece.kind, from, to, self);
                if legal {
                    match piece.kind {
                        PieceKind::Pawn => {
                            //pawns are a special case because they can move up or down, but can only take on the diagonal.
                            distance_information.distance == 2 && (distance_information.move_direction != MoveDirection::Up && distance_information.move_direction != MoveDirection::Down)
                        },
                        _ => true
                    }
                } else {
                    false
                }

            },
            Square::Empty => false //can't threaten another square with an empty square.
        }
    }

    pub fn twixt_hither_and_yon(&self, from: &Coordinates, to: &Coordinates, direction: MoveDirection) -> bool {
        let mut path_clear = true;

        let from_letter_value = from.letter.eval();
        let to_letter_value = to.letter.eval();
        let from_number_value = from.number;
        let to_number_value = to.number;
        match direction {
            MoveDirection::Down => {
                for i in (to_number_value + 1)..(from_number_value) {
                    match self.retreive_square(&Coordinates{ letter: from.letter, number: i}) {
                        Square::Full(piece) => {
                            path_clear = false;
                            break;
                        },
                        Square::Empty => (), //Expected and fine
                    }
                }
            },
            MoveDirection::Up => {
                for i in (from_number_value + 1)..to_number_value {
                    match self.retreive_square(&Coordinates{ letter: from.letter, number: i}) {
                        Square::Full(_) => {
                            path_clear = false;
                            break;
                        },
                        Square::Empty => (), //Expected and fine
                    }
                }
            },
            MoveDirection::Left => {
                for i in (to_letter_value + 1)..from_letter_value {
                    match self.retreive_square(&Coordinates {letter: ColumnLetter::construct_from_usize(i), number: from_number_value}){
                        Square::Full(piece) => {
                            path_clear = false;
                            break;  
                        },
                        Square::Empty => (),
                    }
                }
            },
            MoveDirection::Right => {
                for i in (from_letter_value + 1)..to_letter_value {
                    match self.retreive_square(&Coordinates {letter: ColumnLetter::construct_from_usize(i), number: from_number_value}){
                        Square::Full(piece) => {
                            path_clear = false;
                            break;  
                        },
                        Square::Empty => (),
                    }
                }
            },
            MoveDirection::DownLeft => {
                let distance = from_letter_value - to_letter_value;
                for i in 1..distance {
                    match self.retreive_square(&Coordinates {
                        letter: ColumnLetter::construct_from_usize(from_letter_value - i),
                        number: from_number_value - i
                    }) {
                        Square::Full(piece) => {
                            path_clear = false;
                            break;
                        },
                        Square::Empty => ()
                    }
                }
            },
            MoveDirection::DownRight => {
                let distance = to_letter_value - from_letter_value;
                for i in 1..distance {
                    match self.retreive_square(&Coordinates{letter: ColumnLetter::construct_from_usize(from_letter_value + i), number: from_number_value - i}) {
                        Square::Full(piece) => {
                            path_clear = false;
                            break;
                        },
                        Square::Empty => ()
                    }
                }
                
            },
            MoveDirection::UpLeft => {
                let distance = to_number_value - from_number_value;
                for i in 1..distance {
                    match self.retreive_square(&Coordinates{
                        letter: ColumnLetter::construct_from_usize(from_letter_value - i),
                        number: from_number_value + i
                    }) {
                        Square::Full(piece) => {
                            path_clear = false;
                            break;
                        },
                        Square::Empty => ()
                    }
                }
            },
            MoveDirection::UpRight => {
                let distance = to_letter_value - from_letter_value;
                for i in 1..distance {
                    // we can rely on the distance being equal, otherwise the move is illegal.
                    match self.retreive_square(&Coordinates {letter: ColumnLetter::construct_from_usize(from_letter_value + i), number: from_number_value + i}) {
                        Square::Full(piece) => {
                            path_clear = false;
                            break;
                        },
                        Square::Empty => {}
                    }

                }
            },
            _ => () //if we are here, the move is either a JHook, which is allowed to jump, or illegal,
        }

        path_clear
    }

    pub fn move_piece(&mut self, from: &Coordinates, to: &Coordinates) -> MoveResult {
        let from_square = self.retreive_square(&from);
        match from_square {
            Square::Full(piece) => {
                if piece.color == self.turn {
                    let (move_legal, taking_piece, target_piece_color, target_piece_kind) = parse_move_legality(piece.kind, from, to, self);

                    if move_legal {
                        // if we're moving the king we need to update his coords
                        if piece.kind == PieceKind::King {
                            match piece.color {
                                PieceColor::Black => {
                                    self.black_side_information.update_king_location(to.letter, to.number);
                                },
                                PieceColor::White => {}
                            }
                        }
                        let replacement_square = from_square.clone();
                        self.set_square(&from, Square::Empty);
                        self.set_square(&to, replacement_square);
                        if taking_piece {
                            self.add_piece_to_kills(target_piece_kind, target_piece_color);
                        }
                        match self.turn {
                            PieceColor::Black => self.turn = PieceColor::White,
                            PieceColor::White => self.turn = PieceColor::Black
                        }
                        //now we have to update the coordinates of the king if necessary
                        MoveResult::MoveCompleted
                    } else {
                        MoveResult::MoveIllegal
                    }
                } else {
                    MoveResult::WrongTurn
                }
                }, //
            Square::Empty => MoveResult::EmptySquare
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
        print!("> ");
    }
}

impl fmt::Display for Board {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        let black_score = self.black_side_information.total_taken_pieces();
        let white_score = self.white_side_information.total_taken_pieces();
        let score_equal = black_score == white_score;
        let black_winning = black_score > white_score;
        self.show_taken_pieces(PieceColor::Black);
        if black_winning {
            println!("+{}", black_score - white_score);
        }
        println!("");
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
        if !black_winning && !score_equal{
            println!("+{}", white_score - black_score);
        }
        println!("");
        Ok(())
    }
}
