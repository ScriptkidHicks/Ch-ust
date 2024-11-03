use core::fmt;
use std::slice::Iter;

use crate::{pieces::*, rules::{king_checkmate_state, parse_move_legality, Mate_State}};

pub fn isize_difference(a: isize, b: isize) -> isize {
    isize::abs(a - b)
}

pub fn board_safe_isize_converter(size: isize) -> Result<usize, &'static str> {
    match size {
        0 => Ok(0),
        1 => Ok(1),
        2 => Ok(2),
        3 => Ok(3),
        4 => Ok(4),
        5 => Ok(5),
        6 => Ok(6),
        7 => Ok(7),
        8 => Ok(8),
        _ => Err("invalid value passed")
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
   pub fn eval(&self) -> isize {
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
    //do not widescope this level of letter access. Keep is strictly local
    use ColumnLetter::*;
    static LETTERS: [ColumnLetter; 8] = [A, B, C, D, E, F, G, H];
    LETTERS.iter()
   } 

   pub fn construct_letter_from_isize(size: isize) -> Result<ColumnLetter, &'static str> {
    match size {
        0 => Ok(ColumnLetter::A),
        1 => Ok(ColumnLetter::B),
        2 => Ok(ColumnLetter::C),
        3 => Ok(ColumnLetter::D),
        4 => Ok(ColumnLetter::E),
        5 => Ok(ColumnLetter::F),
        6 => Ok(ColumnLetter::G),
        7 => Ok(ColumnLetter::H),
        _ => {
            if (size > 7) {
                Err("Size larger than 7 in construct_letter_from_size")
            } else {   
                Err("Size less than 0 in construct_letter_from_size")
            }
        }
    }
   }

   pub fn convert_to(letter: char) -> Result<ColumnLetter, &'static str> {
    match letter.to_ascii_lowercase() {
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
    pub distance: isize
}

#[derive(Clone, Copy, PartialEq)]
pub struct Coordinates {
    pub letter: ColumnLetter,
    pub number: isize
}

impl fmt::Display for Coordinates {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        print!("{}{}", self.letter, self.number);
        Ok(())
    }
}

pub fn measure_distance(from: &Coordinates, to: &Coordinates) -> SquareToSquareInformation {
    let mut garnered_move_direction: MoveDirection = MoveDirection::NoMove;
    if from.number < 1 || from.number > 8 || to.number < 0 || to.number > 8 {
        return SquareToSquareInformation {
            move_direction: garnered_move_direction,
            distance: 0
        }
    }
    

    let lat_distance = isize_difference(from.letter.eval(), to.letter.eval()); 

    let vert_distance = isize_difference(from.number, to.number);


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

pub enum DiagonalDirection {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

impl DiagonalDirection {
    pub fn conditional_continue(&self, letter_value: isize, number_value: isize) -> bool {
        match self {
            Self::DownLeft => letter_value > 0 && number_value > 1,
            Self::DownRight => letter_value < 7 && number_value > 1,
            Self::UpLeft => letter_value > 0 && number_value < 7,
            Self::UpRight => letter_value < 7 && number_value < 7,
        }
    }

    pub fn modify_letter_and_number_values(&self, letter_value: &mut isize, number_value: &mut isize) {
        match self {
            Self::DownLeft => {
                *letter_value -= 1;
                *number_value -= 1;
            },
            Self::DownRight => {
                *letter_value += 1;
                *number_value -= 1;
            },
            Self::UpLeft => {
                *letter_value -= 1;
                *number_value += 1;
            },
            Self::UpRight => {
                *letter_value += 1;
                *number_value += 1;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Square {
    Empty,
    Full(Piece)
}

impl Square {
    pub fn get_legal_cross_targets(&self, coordinates: &Coordinates, piece_kind: PieceKind, board: &Board, legal_target_squares: &mut Vec<Coordinates>) {
        for i in 1..9 {
            match ColumnLetter::construct_letter_from_isize(i) {
                Ok(found_letter) => {
                    let column_target = Coordinates{letter: found_letter, number: coordinates.number};
                    let row_target=  Coordinates{ letter: coordinates.letter.clone(), number: i};
                    let (column_move_legal, _, _, _, _, _) = parse_move_legality(piece_kind, &coordinates, &column_target, &board);
                    let (row_move_legal, _, _, _, _, _) = parse_move_legality(piece_kind, &coordinates, &row_target, &board);
                    if column_move_legal {legal_target_squares.push(column_target);}
                    if row_move_legal {legal_target_squares.push(row_target);}
                },
                Err(text) => {
                    panic!("{}", text);
                }
            }
        }
    }

    pub fn get_legal_single_diagonal(&self, coordinates: &Coordinates, direction: DiagonalDirection, piece_kind: PieceKind, board: &Board, legal_target_squares: &mut Vec<Coordinates>) {
        let mut moving_letter_value = coordinates.letter.eval();
        let mut moving_number_value = coordinates.number;

        while DiagonalDirection::conditional_continue(&direction, moving_letter_value, moving_number_value) {
            match ColumnLetter::construct_letter_from_isize(moving_letter_value) {
                Ok(found_letter) => {
                     DiagonalDirection::modify_letter_and_number_values(&direction, &mut moving_letter_value, &mut moving_number_value);
                    let target_coords = Coordinates{letter: found_letter, number: moving_number_value};
                    let (target_legal, _, _, _, _, _) = parse_move_legality(piece_kind, coordinates, &target_coords, board);
                    if target_legal {
                        legal_target_squares.push(target_coords);
                    }
                },
                Err(_) => {
                    break;
                }
            }
        }
    }

    pub fn get_legal_diagonal_targets(&self, coordinates: &Coordinates, piece_kind: PieceKind, board: &Board, legal_target_squares: &mut Vec<Coordinates>){
        self.get_legal_single_diagonal(coordinates, DiagonalDirection::UpRight, piece_kind, board, legal_target_squares);

        self.get_legal_single_diagonal(coordinates, DiagonalDirection::DownRight, piece_kind, board, legal_target_squares);

        self.get_legal_single_diagonal(coordinates, DiagonalDirection::UpLeft, piece_kind, board, legal_target_squares);

        self.get_legal_single_diagonal(coordinates, DiagonalDirection::DownLeft, piece_kind, board, legal_target_squares);
    }

    pub fn get_legal_targets(&self, coordinates: &Coordinates, board: &Board) -> Vec<Coordinates> {
        let mut legal_target_squares: Vec<Coordinates> = Vec::new();

        match self {
            Square::Empty => {//do nothing. No legal targets for an empty square.
                },
            Square::Full(piece) => {
                match piece.kind {
                    PieceKind::Pawn => {
                        match piece.color {
                            PieceColor::Black => {
                                
                            },
                            PieceColor::White => {
                            }
                        };
                    },
                    PieceKind::Rook => {
                        self.get_legal_cross_targets(coordinates, piece.kind, board, &mut legal_target_squares);
                    },
                    PieceKind::Knight => {
                        
                    },
                    PieceKind::Bishop => {
                        self.get_legal_diagonal_targets(coordinates, piece.kind, board, &mut legal_target_squares);
                    },
                    PieceKind::Queen => {
                        self.get_legal_cross_targets(coordinates, piece.kind, board, &mut legal_target_squares);
                        self.get_legal_diagonal_targets(coordinates, piece.kind, board, &mut legal_target_squares);
                    },
                    PieceKind::King => {
                        for row_mod in -1..2 {
                            for col_mod in -1..2 {
                                match ColumnLetter::construct_letter_from_isize(coordinates.letter.eval() + col_mod) {
                                    Ok(new_letter) => {
                                        //ok, we have a new valid column, So lets go check if we can get that square.
                                        let investigating_coordinates = Coordinates{letter: new_letter, number: coordinates.number + row_mod};
                                        let (move_legal, _, _, _, _, _) = parse_move_legality(piece.kind, coordinates, &investigating_coordinates, board);
                                        if move_legal {
                                            legal_target_squares.push(investigating_coordinates);
                                        }
                                    },
                                    Err(_) => {
                                        //That's ok, it's just out of bounds
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        legal_target_squares
    }

    pub fn show_me_legal_squares(&self, coordinates: &Coordinates, board: &Board) {
        let legal_squares = self.get_legal_targets(coordinates, board);
        for target in legal_squares {
            print!("{} ", target);
        }
        println!();
    }
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

    pub fn update_king_location (&mut self, letter: &ColumnLetter, number: &isize) {
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
    CompletedSafely,
    BlackKingCheckmated,
    WhiteKingCheckmated,
    BlackKingChecked,
    WhiteKingChecked,
    Stalemate,
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

    pub fn show_me_legal_squares(&self, coords: &Coordinates) {
        match self.retreive_square(coords) {
            Ok(retrieved_square) => {
                retrieved_square.show_me_legal_squares(coords, self);
            },
            Err(_) => {}
        }
    }

    pub fn search_squares(&self, side_color: PieceColor, callback: fn(&Square, PieceColor, &Board) -> bool) -> bool {
        for row in self.rows.iter() {
            for square in row.squares.iter() {
                if callback(square, side_color, self) {
                    return true
                }
            }
        }

        false
    }
    
    fn convert_row_usize(size: usize) -> Result<usize, &'static str> {
        if (size > 0 && size < 9){
            Ok(7 - (size - 1))
        } else {
            Err("attempted to perform subtraction out of bounds of usize")
        }
    }

    pub fn retreive_square(&self, coords: &Coordinates) -> Result<Square, &'static str> {
        match board_safe_isize_converter(coords.number) {
            Ok(usize_number) => {
                match board_safe_isize_converter(coords.letter.eval()) {
                    Ok(usize_letter) => {
                        match Self::convert_row_usize(usize_number) {
                            Ok(converted_size) => {
                                Ok((self.rows[converted_size].squares[usize_letter]).clone())
                            },
                            Err(error_text) => {
                                Err(error_text)
                            }
                        }
                    },
                    Err(letter_text) => Err(letter_text) 
                }
            },
            Err(number_text) => Err(number_text) 
        }
    }

    pub fn set_square(&mut self, coords: &Coordinates, square: Square) {
        match board_safe_isize_converter(coords.number) {
            Ok(usize_number) => {
                match board_safe_isize_converter(coords.letter.eval()) {
                    Ok(usize_letter) => {
                        match Self::convert_row_usize(usize_number) {
                            Ok(converted_rowsize) => {
                                self.rows[converted_rowsize].squares[usize_letter] = square;
                            },
                            Err(text) => {}
                        }
                    },
                    Err(letter_text) => ()
                }
            },
            Err(_) => ()
        }
    }

    pub fn king_can_castle (&self, king_color: PieceColor, is_kingside_query: bool) -> bool {
        if self.is_king_in_danger(king_color) {
            //now now, no castling out of check.
            return false;
        }
        match king_color {
            PieceColor::Black => self.black_side_information.king_can_castle(is_kingside_query) && self.is_castling_safe(king_color, is_kingside_query),
            PieceColor::White => self.white_side_information.king_can_castle(is_kingside_query) && self.is_castling_safe(king_color, is_kingside_query)
        }
    }

    fn is_castling_safe(&self, king_color: PieceColor, is_kingside_query: bool) -> bool {
        //logical shorting means that we can safely blind check.
        let mut copied_board = self.clone();

        let row_number: isize = match king_color {
            PieceColor::Black => 8,
            PieceColor::White => 1
        };

        let empty_square = Square::Empty;
        match copied_board.retreive_square(&Coordinates{letter: ColumnLetter::E, number: row_number}) {
            Ok(found_square) => {
                let moved_square = found_square.clone();
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
            },
            Err(_) => {
                panic!("You failed to retreive the square the king should be on at E{}", row_number)
            }
        }
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
                match self.retreive_square(&from_coords) {
                    Ok(gotten_square) => {
                        match gotten_square {
                            Square::Full(piece ) => {
                                // we only care about the ability of other pieces to tkae our king;
                                if piece.color != king_color {
                                    //only pieces of the opposite color can threaten the king
                                    if self.square_threatens_square(&from_coords, &target_king_coordinates) {
                                        // oops, we found a square that threatens the king. Can't allow that!
                                        return true;
                                    }
                                }
                            },
                            Square::Empty => ()
                        }
                    },
                    Err(text) => {
                        panic!("oops we tried to retrieve an illegal square in is_king_in_danger with error {}", text)
                    }
                }
            }
        }

        false
    }

    pub fn square_threatens_square(&self, from: &Coordinates, to: &Coordinates) -> bool {
        match self.retreive_square(from) {
            Ok(found_square) => {
                match found_square {
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
            },
            Err(_) => {
                panic!("oops, we tried to check if a square threatens another but that square was illegal!")
            }
        }
    }

    fn panic_in_twixt_hither() {
        panic!("attempting to access square illegally in ")
    }

    fn inner_path_clear_checking(&self, path_clear: &mut bool, coords: &Coordinates) -> bool {
        match self.retreive_square(coords) {
            Ok(found_square) => {
                match found_square {
                    Square::Full(_) => {
                        *path_clear = false;
                        false
                    },
                    Square::Empty => true, //Expected and fine
                }
            },
            Err(_) => {
                Self::panic_in_twixt_hither();
                false
            }
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
                    if !self.inner_path_clear_checking(&mut path_clear, &Coordinates{ letter: from.letter, number: i}) {
                        break;
                    }
                }
            },
            MoveDirection::Up => {
                for i in (from_number_value + 1)..to_number_value {
                    if !self.inner_path_clear_checking(&mut path_clear, &Coordinates{ letter: from.letter, number: i}) {
                        break;
                    }
                }
            },
            MoveDirection::Left => {
                for i in (to_letter_value + 1)..from_letter_value {
                    match ColumnLetter::construct_letter_from_isize(i) {
                        Ok(found_letter) => {
                            if !self.inner_path_clear_checking(&mut path_clear, &Coordinates {letter: found_letter, number: from_number_value}) {
                                break;
                            }
                        },
                        Err(_) => {
                            panic!("attempted to access out of bounds");
                        }
                    }
                }
            },
            MoveDirection::Right => {
                for i in (from_letter_value + 1)..to_letter_value {
                    match ColumnLetter::construct_letter_from_isize(i) {
                        Ok(found_letter) => {
                            if !self.inner_path_clear_checking(&mut path_clear, &Coordinates {letter: found_letter, number: from_number_value}) {
                                break;
                            }
                        },
                        Err(_) => {
                            panic!("attempted to acces out of bounds");
                        }
                    }
                }
            },
            MoveDirection::DownLeft => {
                let distance = from_letter_value - to_letter_value;
                for i in 1..distance {
                    match ColumnLetter::construct_letter_from_isize(from_letter_value - i) {
                        Ok(found_letter) => {
                            if self.inner_path_clear_checking(&mut path_clear, &Coordinates {
                                letter: found_letter,
                                number: from_number_value - i
                            }) {
                                break;
                            }
                        },
                        Err(_) => {
                            panic!("attempted to access out of bounds")
                        }
                    }
                }
            },
            MoveDirection::DownRight => {
                let distance = to_letter_value - from_letter_value;
                for i in 1..distance {
                    match ColumnLetter::construct_letter_from_isize(from_letter_value + i) {
                        Ok(found_letter) => {
                            if !self.inner_path_clear_checking(&mut path_clear, &Coordinates{letter: found_letter, number: from_number_value - i}) {
                                break;
                            }
                        },
                        Err(_) => {
                            panic!("attempted to access out of bounds")
                        }
                    }
                }
                
            },
            MoveDirection::UpLeft => {
                let distance = to_number_value - from_number_value;
                for i in 1..distance {
                    match ColumnLetter::construct_letter_from_isize(from_letter_value - i) {
                        Ok(found_letter) => {
                            if !self.inner_path_clear_checking(&mut path_clear, &Coordinates{
                        letter: found_letter,
                        number: from_number_value + i
                    }) {
                        break;
                    }
                        },
                        Err(_) => {panic!("attempted to access out of bounds")}
                    }
                }
            },
            MoveDirection::UpRight => {
                let distance = to_letter_value - from_letter_value;
                for i in 1..distance {
                    // we can rely on the distance being equal, otherwise the move is illegal.
                    match ColumnLetter::construct_letter_from_isize(from_letter_value + i) {
                        Ok(found_letter) => {
                            if !self.inner_path_clear_checking(&mut path_clear, &Coordinates {letter: found_letter, number: from_number_value + i}) {
                                break;
                            }
                        }, 
                        Err(_) => {
                            panic!("attempted to access out of bounds")
                        }
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
        let opt_pulled_king = self.retreive_square(&Coordinates{letter: ColumnLetter::E, number: row_number}).clone();
        let opt_pulled_rook = match is_kingside { 
            true => self.retreive_square(&Coordinates{letter: ColumnLetter::H, number: row_number}).clone(), 
            false => self.retreive_square(&Coordinates{letter: ColumnLetter::A, number: row_number}).clone()
        };
        let empty_square = Square::Empty;

        match opt_pulled_king {
            Ok(pulled_king) => {
                match opt_pulled_rook {
                    Ok(pulled_rook) => {
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
                    },
                    Err(_) => {
                        panic!("attempted to castle, but could not retreive rook")
                    }
                }
            },
            Err(_) => {
                panic!("attempted to castle, but could not retreive king")
            }
        }
    }

    pub fn move_piece(&mut self, from: &Coordinates, to: &Coordinates) -> MoveResult {
        let move_result: MoveResult;
        let opt_from_square = self.retreive_square(&from);
        match opt_from_square {
            Ok(from_square) => {
                let replacement_square = from_square.clone();
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
                                let opponent_color = match piece.color {
                                    PieceColor::Black => PieceColor::White,
                                    PieceColor::White => PieceColor::Black
                                };
                                //we can make the move they are requesting. Lets check what state this leaves the board in.
                                match king_checkmate_state(opponent_color, &self) {
                                    Mate_State::Check => {
                                        match opponent_color {
                                            PieceColor::Black => {
                                                move_result = MoveResult::BlackKingChecked;
                                            },
                                            PieceColor::White => {
                                                move_result = MoveResult::WhiteKingChecked;
                                            }
                                        }
                                    }
                                    Mate_State::CheckMate => {
                                        match opponent_color {
                                            PieceColor::Black => {move_result = MoveResult::BlackKingCheckmated},
                                            PieceColor::White => {move_result = MoveResult::WhiteKingCheckmated}
                                        }
                                    },
                                    Mate_State::StaleMate => {
                                        move_result = MoveResult::Stalemate;
                                    },
                                    Mate_State::Safe => {
                                        move_result = MoveResult::CompletedSafely;
                                    }
                                }
                            } else {
                                move_result = MoveResult::MoveIllegal;
                            }
                        } else {
                            move_result = MoveResult::WrongTurn;
                        }
                        }, //
                    Square::Empty => { 
                        move_result = MoveResult::EmptySquare;
                    }
                }
            },
            Err(_) => {
                move_result = MoveResult::MoveIllegal;
            }
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
