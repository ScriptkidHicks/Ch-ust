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

   pub fn construct_letter_from_usize(size: usize) -> ColumnLetter {
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

#[derive(Clone, Copy, PartialEq)]
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

impl Coordinates {
    pub fn increment_column(original_coordinates: Coordinates) -> Coordinates {
        Coordinates {
            letter: ColumnLetter::construct_letter_from_usize(original_coordinates.letter.eval() + 1),
            number: original_coordinates.number
        }
    }

    pub fn decrement_column(original_coordinates: Coordinates) -> Coordinates {
        Coordinates {
            letter: ColumnLetter::construct_letter_from_usize(original_coordinates.letter.eval() - 1),
            number: original_coordinates.number
        }
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

    pub fn update_king_location (&mut self, letter: &ColumnLetter, number: &usize) {
        self.current_king_square = Coordinates {letter: *letter, number: *number};
        //moving your king at all negats your ability to castle on both sides.
        self.can_castle_kingside = false;
        self.can_castle_queenside = false;
    }

    pub fn king_can_castle (&self, is_kingside_query: bool) -> bool {
        if is_kingside_query { self.can_castle_kingside } else { self.can_castle_queenside }
    }

    pub fn remove_castling_rights(&mut self, is_kingside: bool) {
        if is_kingside {self.can_castle_kingside = false;} else {self.can_castle_queenside = false;} 
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

    pub fn king_can_castle (&self, king_color: PieceColor, is_kingside_query: bool) -> bool {
        match king_color {
            PieceColor::Black => self.black_side_information.king_can_castle(is_kingside_query) && self.is_castling_safe(king_color, is_kingside_query),
            PieceColor::White => self.white_side_information.king_can_castle(is_kingside_query) && self.is_castling_safe(king_color, is_kingside_query)
        }
    }

    fn is_castling_safe(&self, king_color: PieceColor, is_kingside_query: bool) -> bool {
        //logical shorting means that we can safely blind check.
        let mut copied_board = self.clone();

        let row_number: usize = match king_color {
            PieceColor::Black => 8,
            PieceColor::White => 1
        };

        let empty_square = Square::Empty;
        let moved_square = copied_board.retreive_square(&Coordinates{letter: ColumnLetter::E, number: row_number}).clone();

        if is_kingside_query {
            copied_board.set_square(&Coordinates{letter: ColumnLetter::E, number: row_number}, empty_square);
            copied_board.set_square(&Coordinates{letter: ColumnLetter::F, number: row_number}, moved_square);
            if copied_board.is_king_in_danger(king_color) {
                //keep your functions flat and return early.
                return false;
            }
            //ok, so we know that it's safe to keep going. Lets check the next state.
            copied_board.set_square(&Coordinates{letter: ColumnLetter::F, number: row_number}, empty_square);
            copied_board.set_square(&Coordinates{letter: ColumnLetter::G, number: row_number}, moved_square);
            if copied_board.is_king_in_danger(king_color) {
                return false;
            }
            // we don't need to check hopping the rook over the king, because there isn't a way to threaten the king by moving that rook.
        } else {
            copied_board.set_square(&Coordinates{letter: ColumnLetter::E, number: row_number}, empty_square);
            copied_board.set_square(&Coordinates{letter: ColumnLetter::D, number: row_number}, moved_square);
            if copied_board.is_king_in_danger(king_color) {
                //keep your functions flat and return early.
                return false;
            }
            //ok, so we know that it's safe to keep going. Lets check the next state.
            copied_board.set_square(&Coordinates{letter: ColumnLetter::D, number: row_number}, empty_square);
            copied_board.set_square(&Coordinates{letter: ColumnLetter::C, number: row_number}, moved_square);
            if copied_board.is_king_in_danger(king_color) {
                return false;
            }
        }


        //if we made it here, then it's safe to return true.
        true
    }

    pub fn remove_castling_rights(&mut self, side_color: PieceColor, is_kingside: bool) {
        match side_color {
            PieceColor::Black => self.black_side_information.remove_castling_rights(is_kingside),
            PieceColor::White => self.white_side_information.remove_castling_rights(is_kingside),
        }
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
                let (legal, _, _, _, _, _) = parse_move_legality(piece.kind, from, to, self);
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
                    match self.retreive_square(&Coordinates {letter: ColumnLetter::construct_letter_from_usize(i), number: from_number_value}){
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
                    match self.retreive_square(&Coordinates {letter: ColumnLetter::construct_letter_from_usize(i), number: from_number_value}){
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
                        letter: ColumnLetter::construct_letter_from_usize(from_letter_value - i),
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
                    match self.retreive_square(&Coordinates{letter: ColumnLetter::construct_letter_from_usize(from_letter_value + i), number: from_number_value - i}) {
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
                        letter: ColumnLetter::construct_letter_from_usize(from_letter_value - i),
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
                    match self.retreive_square(&Coordinates {letter: ColumnLetter::construct_letter_from_usize(from_letter_value + i), number: from_number_value + i}) {
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

    fn update_king_location(&mut self, coords: Coordinates, king_color: PieceColor) {
        match king_color {
            PieceColor::Black => self.black_side_information.update_king_location(&coords.letter, &coords.number),
            PieceColor::White => self.white_side_information.update_king_location(&coords.letter, &coords.number),
        }
    }

    fn castle(&mut self, king_color: PieceColor, is_kingside: bool) {
        let row_number = match king_color {
            PieceColor::White => 1, 
            PieceColor::Black => 8
        };
        let pulled_king = self.retreive_square(&Coordinates{letter:E, number: row_number}).clone();
        let pulled_rook = match is_kingside { 
            true => self.retreive_square(&Coordinates{letter: ColumnLetter::H, number: row_number}).clone(), 
            false => self.retreive_square(&Coordinates{letter: ColumnLetter::A, number: row_number}).clone()
        };
        let empty_square = Square::Empty;

        match is_kingside {
            true => {
                self.set_square(&Coordinates{letter: ColumnLetter::H, number: row_number}, empty_square);
                self.set_square(&Coordinates{letter: ColumnLetter::E, number: row_number}, empty_square);
                self.set_square(&Coordinates{letter: ColumnLetter::G, number: row_number}, pulled_king);
                self.set_square(&Coordinates{letter: ColumnLetter::F, number: row_number}, pulled_rook);
            },
            false => {
                self.set_square(&Coordinates{letter: ColumnLetter::A, number: row_number}, empty_square);
                self.set_square(&Coordinates{letter: ColumnLetter::E, number: row_number}, empty_square);
                self.set_square(&Coordinates{letter: ColumnLetter::C, number: row_number}, pulled_king);
                self.set_square(&Coordinates{letter: ColumnLetter::D, number: row_number}, pulled_rook);
            }
        };
    }

    pub fn move_piece(&mut self, from: &Coordinates, to: &Coordinates) -> MoveResult {
        let from_square = self.retreive_square(&from).clone();
        let replacement_square = from_square.clone();
        let move_result: MoveResult;
        match from_square {
            Square::Full(piece) => {
                if piece.color == self.turn {
                    let (move_legal, taking_piece, target_piece_color, target_piece_kind, move_direction, move_distance) = parse_move_legality(piece.kind, from, to, self);

                    if move_legal {
                           // if we're moving the king we need to update his coords
                        match piece.kind {
                            PieceKind::King => {
                                // if we got here, the rule checker already knows that this move is safe and legal. Lets check if we're castling, then update appropriately
                                if move_distance == 2 {
                                    if move_direction == MoveDirection::Right {
                                        self.castle(piece.color, true);
                                    } else if move_direction == MoveDirection::Left {
                                        self.castle(piece.color, false);
                                    }
                                }
                                self.update_king_location(*from, piece.color);
                            },
                            PieceKind::Rook => {
                                // we need to check if they're moving off their original square, and negate castling rights as necessary.
                                if from.letter == ColumnLetter::A {
                                    //we can just do this dumbly, since it doesn't cost much, and firing it off every time ensures safety.
                                    //Lesson: sometimes it's cheaper to just ensure bool state than it is to check every time.
                                    self.remove_castling_rights(piece.color, false);
                                } else if from.letter == ColumnLetter::H {
                                    self.remove_castling_rights(piece.color, true);
                                }
                            },
                            _ => {}
                        }
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
                        move_result = MoveResult::MoveCompleted
                    } else {
                        move_result = MoveResult::MoveIllegal
                    }
                } else {
                    move_result = MoveResult::WrongTurn
                }
                }, //
            Square::Empty => move_result = MoveResult::EmptySquare
        }

        move_result
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
