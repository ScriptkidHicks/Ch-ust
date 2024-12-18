use core::fmt;
use std::{slice::Iter, thread, time};

use crate::{
    pieces::*,
    rules::{king_checkmate_state, parse_move_legality, MateState},
};

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
        _ => Err("invalid value passed"),
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
    H,
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
            ColumnLetter::H => 7,
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
                if size > 7 {
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
            _ => Err("Not a valid col"),
        }
    }
}

impl fmt::Display for ColumnLetter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DiagonalDirection {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
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

    pub fn modify_letter_and_number_values(
        &self,
        letter_value: &mut isize,
        number_value: &mut isize,
    ) {
        match self {
            Self::DownLeft => {
                *letter_value -= 1;
                *number_value -= 1;
            }
            Self::DownRight => {
                *letter_value += 1;
                *number_value -= 1;
            }
            Self::UpLeft => {
                *letter_value -= 1;
                *number_value += 1;
            }
            Self::UpRight => {
                *letter_value += 1;
                *number_value += 1;
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
    Diagonal(DiagonalDirection),
    JHook,
    NoMove,
    IllegalMove,
}

impl fmt::Display for MoveDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Down => write!(f, "Down"),
            Self::Up => write!(f, "Up"),
            Self::Right => write!(f, "Right"),
            Self::Left => write!(f, "Left"),
            Self::JHook => write!(f, "JHook"),
            Self::Diagonal(diag) => match diag {
                DiagonalDirection::DownLeft => write!(f, "Down Left"),
                DiagonalDirection::DownRight => write!(f, "Down Right"),
                DiagonalDirection::UpLeft => write!(f, "Up Left"),
                DiagonalDirection::UpRight => write!(f, "Up Right"),
            },
            Self::IllegalMove => write!(f, "Illegal Move"),
            Self::NoMove => write!(f, "No Move"),
        }
    }
}

pub struct SquareToSquareInformation {
    pub move_direction: MoveDirection,
    pub distance: isize,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Coordinates {
    pub letter: ColumnLetter,
    pub number: isize,
}

impl Coordinates {
    pub fn new(input_letter: ColumnLetter, input_row: isize) -> Coordinates {
        Coordinates {
            letter: input_letter,
            number: input_row,
        }
    }
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
            distance: 0,
        };
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
                    garnered_move_direction = MoveDirection::Diagonal(DiagonalDirection::DownLeft);
                } else {
                    garnered_move_direction = MoveDirection::Diagonal(DiagonalDirection::DownRight);
                }
            } else {
                // we know the move is up.
                if from.letter.eval() > to.letter.eval() {
                    garnered_move_direction = MoveDirection::Diagonal(DiagonalDirection::UpLeft);
                } else {
                    garnered_move_direction = MoveDirection::Diagonal(DiagonalDirection::UpRight);
                }
            }
        } else if (lat_distance == 1 && vert_distance == 2)
            || (vert_distance == 1 && lat_distance == 2)
        {
            garnered_move_direction = MoveDirection::JHook;
        } else {
            garnered_move_direction = MoveDirection::IllegalMove;
        }
    }

    SquareToSquareInformation {
        move_direction: garnered_move_direction,
        distance: lat_distance + vert_distance,
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Square {
    Empty,
    Full(Piece),
}

impl Square {
    pub fn get_legal_cross_targets(
        &self,
        coordinates: &Coordinates,
        board: &Board,
        legal_target_squares: &mut Vec<Coordinates>,
    ) {
        for i in 0..8 {
            match ColumnLetter::construct_letter_from_isize(i) {
                Ok(found_letter) => {
                    self.get_legal_single_target(
                        coordinates,
                        found_letter,
                        coordinates.number,
                        board,
                        legal_target_squares,
                    );
                    self.get_legal_single_target(
                        coordinates,
                        coordinates.letter,
                        i,
                        board,
                        legal_target_squares,
                    );
                }
                Err(text) => {
                    panic!("{}", text);
                }
            }
        }
    }

    pub fn get_legal_single_diagonal(
        &self,
        coordinates: &Coordinates,
        direction: DiagonalDirection,
        board: &Board,
        legal_target_squares: &mut Vec<Coordinates>,
    ) {
        let mut moving_letter_value = coordinates.letter.eval();
        let mut moving_number_value = coordinates.number;

        while DiagonalDirection::conditional_continue(
            &direction,
            moving_letter_value,
            moving_number_value,
        ) {
            match ColumnLetter::construct_letter_from_isize(moving_letter_value) {
                Ok(found_letter) => {
                    DiagonalDirection::modify_letter_and_number_values(
                        &direction,
                        &mut moving_letter_value,
                        &mut moving_number_value,
                    );
                    self.get_legal_single_target(
                        coordinates,
                        found_letter,
                        moving_number_value,
                        board,
                        legal_target_squares,
                    );
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    pub fn get_legal_diagonal_targets(
        &self,
        coordinates: &Coordinates,
        board: &Board,
        legal_target_squares: &mut Vec<Coordinates>,
    ) {
        self.get_legal_single_diagonal(
            coordinates,
            DiagonalDirection::UpRight,
            board,
            legal_target_squares,
        );

        self.get_legal_single_diagonal(
            coordinates,
            DiagonalDirection::DownRight,
            board,
            legal_target_squares,
        );

        self.get_legal_single_diagonal(
            coordinates,
            DiagonalDirection::UpLeft,
            board,
            legal_target_squares,
        );

        self.get_legal_single_diagonal(
            coordinates,
            DiagonalDirection::DownLeft,
            board,
            legal_target_squares,
        );
    }

    pub fn get_legal_single_target(
        &self,
        from: &Coordinates,
        col_letter: ColumnLetter,
        row_number: isize,
        board: &Board,
        legal_target_squares: &mut Vec<Coordinates>,
    ) {
        let investigating_coordinates = Coordinates {
            letter: col_letter,
            number: row_number,
        };
        let (move_legal, _, _, _, _, _, _, _) =
            parse_move_legality(from, &investigating_coordinates, board);
        if move_legal {
            legal_target_squares.push(investigating_coordinates);
        }
    }

    pub fn get_legal_targets(&self, coordinates: &Coordinates, board: &Board) -> Vec<Coordinates> {
        let mut legal_target_squares: Vec<Coordinates> = Vec::new();

        match self {
            Square::Empty => { //do nothing. No legal targets for an empty square.
            }
            Square::Full(piece) => {
                match piece.kind {
                    PieceKind::Pawn => {
                        let single_step: isize;
                        let double_step: isize;
                        match piece.color {
                            PieceColor::Black => {
                                single_step = -1;
                                double_step = -2;
                            }
                            PieceColor::White => {
                                single_step = 1;
                                double_step = -2;
                            }
                        };

                        //regular step forward
                        self.get_legal_single_target(
                            coordinates,
                            coordinates.letter,
                            coordinates.number + single_step,
                            board,
                            &mut legal_target_squares,
                        );

                        //double jump. We can let parsing handle the legality.
                        self.get_legal_single_target(
                            coordinates,
                            coordinates.letter,
                            coordinates.number + double_step,
                            board,
                            &mut legal_target_squares,
                        );

                        //now lets try the two attack vectors
                        let attack_vectors: Vec<isize> = vec![-1, 1];
                        for vector in attack_vectors {
                            match ColumnLetter::construct_letter_from_isize(
                                coordinates.letter.eval() + vector,
                            ) {
                                Ok(new_letter) => {
                                    self.get_legal_single_target(
                                        coordinates,
                                        new_letter,
                                        coordinates.number + single_step,
                                        board,
                                        &mut legal_target_squares,
                                    );
                                }
                                Err(_) => {
                                    //this is fine. It's just on the edge
                                }
                            }
                        }
                    }
                    PieceKind::Rook => {
                        self.get_legal_cross_targets(coordinates, board, &mut legal_target_squares);
                    }
                    PieceKind::Knight => {
                        let row_alters: Vec<(isize, isize)> =
                            vec![(-2, -1), (-2, 1), (2, -1), (2, 1)];
                        for (alter_a, alter_b) in row_alters {
                            match ColumnLetter::construct_letter_from_isize(
                                coordinates.letter.eval() + alter_a,
                            ) {
                                Ok(new_letter) => {
                                    self.get_legal_single_target(
                                        coordinates,
                                        new_letter,
                                        coordinates.number + alter_b,
                                        board,
                                        &mut legal_target_squares,
                                    );
                                }
                                Err(_) => {
                                    //oops, out of bounds
                                }
                            }
                            match ColumnLetter::construct_letter_from_isize(
                                coordinates.letter.eval() + alter_b,
                            ) {
                                Ok(new_letter) => {
                                    self.get_legal_single_target(
                                        coordinates,
                                        new_letter,
                                        coordinates.number + alter_a,
                                        board,
                                        &mut legal_target_squares,
                                    );
                                }
                                Err(_) => {
                                    //oops, out of bounds
                                }
                            }
                        }
                    }
                    PieceKind::Bishop => {
                        self.get_legal_diagonal_targets(
                            coordinates,
                            board,
                            &mut legal_target_squares,
                        );
                    }
                    PieceKind::Queen => {
                        self.get_legal_cross_targets(coordinates, board, &mut legal_target_squares);
                        self.get_legal_diagonal_targets(
                            coordinates,
                            board,
                            &mut legal_target_squares,
                        );
                    }
                    PieceKind::King => {
                        for row_mod in -1..2 {
                            for col_mod in -1..2 {
                                match ColumnLetter::construct_letter_from_isize(
                                    coordinates.letter.eval() + col_mod,
                                ) {
                                    Ok(new_letter) => {
                                        //ok, we have a new valid column, So lets go check if we can get that square.
                                        self.get_legal_single_target(
                                            coordinates,
                                            new_letter,
                                            coordinates.number + row_mod,
                                            board,
                                            &mut legal_target_squares,
                                        );
                                    }
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

    pub fn get_fen_value(&self) -> String {
        match self {
            Self::Full(piece) => piece.get_fen_string(),
            Square::Empty => {
                //we will just return an empty string if the square is empty, and handle it higher up
                "".to_string()
            }
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "[  ]"),
            Self::Full(piece) => write!(f, "[{}{}]", piece.color, piece.kind),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Row {
    squares: [Square; 8],
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
            squares: [Square::Empty; 8],
        }
    }

    pub fn new(input_squares: [Square; 8]) -> Row {
        Row {
            squares: input_squares,
        }
    }

    pub fn pawn_row(piece_color: PieceColor) -> Self {
        Row {
            squares: [Square::Full(Piece {
                color: piece_color,
                kind: PieceKind::Pawn,
            }); 8],
        }
    }

    pub fn default_back_row(piece_color: PieceColor) -> Self {
        Row {
            squares: [
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Rook,
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Knight,
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Bishop,
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Queen,
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::King,
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Bishop,
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Knight,
                }),
                Square::Full(Piece {
                    color: piece_color,
                    kind: PieceKind::Rook,
                }),
            ],
        }
    }

    pub fn set_square(&mut self, index: usize, square: Square) {
        self.squares[index] = square;
    }

    pub fn generate_row_fen_string(&self) -> String {
        let mut accum_string = String::new();
        let mut empty_square_count = 0;

        for square in self.squares.iter() {
            let square_string = square.get_fen_value();

            if square_string == "" {
                empty_square_count += 1;
            } else {
                if empty_square_count != 0 {
                    accum_string.push_str(empty_square_count.to_string().as_str());
                    empty_square_count = 0;
                }

                accum_string.push_str(&square_string);
            }
        }
        if empty_square_count != 0 {
            accum_string.push_str(empty_square_count.to_string().as_str());
        }
        accum_string
    }
}

#[derive(Clone, PartialEq)]
pub struct SideInformation {
    taken_pieces: Vec<PieceKind>,
    can_castle_kingside: bool,
    can_castle_queenside: bool,
    current_king_square: Coordinates,
}

impl fmt::Display for SideInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kingside castling: {:?}\n", self.can_castle_kingside);
        write!(f, "queenside castling: {:?}\n", self.can_castle_queenside);
        println!("king location: {}", self.current_king_square);
        Ok(())
    }
}

impl SideInformation {
    pub fn default(king_color: PieceColor) -> Self {
        SideInformation {
            taken_pieces: Vec::new(),
            can_castle_kingside: true,
            can_castle_queenside: true,
            current_king_square: Coordinates {
                letter: ColumnLetter::E,
                number: match king_color {
                    PieceColor::Black => 8,
                    PieceColor::White => 1,
                },
            },
        }
    }

    pub fn adjust_taken_pieces(
        &mut self,
        taken_pawns: i32,
        taken_rooks: i32,
        taken_knights: i32,
        taken_bishops: i32,
        taken_queens: i32,
    ) {
        self.taken_pieces.clear();

        for _ in 0..taken_pawns {
            self.taken_pieces.push(PieceKind::Pawn);
        }

        for _ in 0..taken_rooks {
            self.taken_pieces.push(PieceKind::Rook);
        }

        for _ in 0..taken_knights {
            self.taken_pieces.push(PieceKind::Knight);
        }

        for _ in 0..taken_bishops {
            self.taken_pieces.push(PieceKind::Bishop);
        }

        for _ in 0..taken_queens {
            self.taken_pieces.push(PieceKind::Queen);
        }
    }

    pub fn sort_taken_pieces(&mut self) {
        let mut pawn_count = 0;
        let mut rook_count = 0;
        let mut knight_count = 0;
        let mut bishop_count = 0;
        let mut queen_count = 0;

        for piece in self.taken_pieces.iter() {
            match piece {
                PieceKind::Pawn => {
                    pawn_count += 1;
                }
                PieceKind::Rook => {
                    rook_count += 1;
                }
                PieceKind::Knight => {
                    knight_count += 1;
                }
                PieceKind::Bishop => {
                    bishop_count += 1;
                }
                PieceKind::Queen => {
                    queen_count += 1;
                }
                PieceKind::King => {
                    panic!("Oops! A king somehow made its way into your taken pieces!");
                }
            }
        }

        self.taken_pieces.clear();

        for _ in 0..pawn_count {
            self.taken_pieces.push(PieceKind::Pawn);
        }

        for _ in 0..rook_count {
            self.taken_pieces.push(PieceKind::Rook);
        }

        for _ in 0..knight_count {
            self.taken_pieces.push(PieceKind::Knight);
        }

        for _ in 0..bishop_count {
            self.taken_pieces.push(PieceKind::Bishop);
        }

        for _ in 0..queen_count {
            self.taken_pieces.push(PieceKind::Queen);
        }
    }

    pub fn generate_fen_string(&self) -> String {
        let mut accum_string = String::new();

        if self.can_castle_kingside {
            accum_string.push('k');
        }

        if self.can_castle_queenside {
            accum_string.push('q');
        }

        accum_string
    }

    pub fn set_castling_rights(&mut self, kingside: bool, queenside: bool) {
        self.can_castle_kingside = kingside;
        self.can_castle_queenside = queenside;
    }

    pub fn add_taken_piece(&mut self, piece_kind: PieceKind) {
        self.taken_pieces.push(piece_kind);
        self.sort_taken_pieces();
    }

    pub fn total_taken_pieces(&self) -> u32 {
        let mut total_value = 0;
        for piece in self.taken_pieces.iter() {
            total_value += piece.get_value()
        }
        total_value
    }

    pub fn update_king_location(&mut self, letter: &ColumnLetter, number: &isize) {
        self.current_king_square = Coordinates {
            letter: *letter,
            number: *number,
        };
        //moving your king at all negats your ability to castle on both sides.
        self.can_castle_kingside = false;
        self.can_castle_queenside = false;
    }

    pub fn king_can_castle(&self, is_kingside_query: bool) -> bool {
        if is_kingside_query {
            self.can_castle_kingside
        } else {
            self.can_castle_queenside
        }
    }

    pub fn remove_castling_rights(&mut self, is_kingside: bool) {
        if is_kingside {
            self.can_castle_kingside = false;
        } else {
            self.can_castle_queenside = false;
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MoveResult {
    CompletedSafely,
    BlackKingCheckmated,
    WhiteKingCheckmated,
    BlackKingChecked,
    WhiteKingChecked,
    Stalemate,
    WrongTurn,
    MoveIllegal,
    EmptySquare,
}

impl fmt::Display for MoveResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq)]
pub struct Board {
    rows: [Row; 8],
    turn: PieceColor,
    opt_legal_passant_square: Option<Coordinates>,
    white_side_information: SideInformation,
    black_side_information: SideInformation,
    half_turns: u32,
    full_turns: u32,
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
                Row::default_back_row(PieceColor::White),
            ],
            turn: PieceColor::White,
            opt_legal_passant_square: None,
            white_side_information: SideInformation::default(PieceColor::White),
            black_side_information: SideInformation::default(PieceColor::Black),
            half_turns: 0,
            full_turns: 1,
        }
    }

    pub fn new(
        input_rows: [Row; 8],
        current_turn: PieceColor,
        input_passant_square: Option<Coordinates>,
        input_white_side: SideInformation,
        input_black_side: SideInformation,
        input_half_turns: u32,
        input_full_turns: u32,
    ) -> Board {
        Board {
            rows: input_rows,
            turn: current_turn,
            opt_legal_passant_square: input_passant_square,
            white_side_information: input_white_side,
            black_side_information: input_black_side,
            half_turns: input_half_turns,
            full_turns: input_full_turns,
        }
    }

    pub fn generate_fen_string(&self) -> String {
        let mut fen_output = String::new();

        for (index, row) in self.rows.iter().enumerate() {
            let mut rowstring = row.generate_row_fen_string();
            if index == 7 {
                rowstring.push(' ');
            } else {
                rowstring.push('/');
            }
            fen_output.push_str(rowstring.as_str());
        }

        //ok, we've dealt with rows, lets try doing the turn
        fen_output.push_str(match self.turn {
            PieceColor::Black => "b ",
            PieceColor::White => "w ",
        });

        //alright, now lets go get the castling rights.
        let mut castling_rights = String::new();

        castling_rights.push_str(
            &self
                .white_side_information
                .generate_fen_string()
                .to_uppercase(),
        );

        castling_rights.push_str(&self.black_side_information.generate_fen_string());

        if (castling_rights.is_empty()) {
            castling_rights.push('-');
        }

        castling_rights.push(' ');

        fen_output.push_str(&castling_rights);

        //ok, now lets see if there's an en passant square.
        match self.get_opt_passant_square() {
            Some(passant_square) => {
                fen_output.push_str(passant_square.letter.to_string().to_lowercase().as_str());
                fen_output.push_str(passant_square.number.to_string().as_str());
                fen_output.push(' ');
            }
            None => {
                fen_output.push_str("- ");
            }
        }

        //finally lets get half and full turns.
        fen_output.push_str(self.half_turns.to_string().as_str());
        fen_output.push(' ');

        fen_output.push_str(self.full_turns.to_string().as_str());

        //and we're done

        fen_output
    }

    pub fn display_sides_information(&self) {
        println!("White Side:");
        println!("{}", self.white_side_information);
        println!("Black Side");
        println!("{}", self.black_side_information);
    }

    pub fn get_turn(&self) -> PieceColor {
        self.turn
    }

    pub fn get_opt_passant_square(&self) -> Option<Coordinates> {
        self.opt_legal_passant_square
    }

    pub fn get_turn_full(&self) -> &'static str {
        match self.turn {
            PieceColor::White => "white",
            PieceColor::Black => "black",
        }
    }

    pub fn board_coords() -> Vec<Coordinates> {
        let mut return_vector: Vec<Coordinates> = Vec::new();

        for row_number in 1..9 {
            for current_letter in ColumnLetter::iterator() {
                return_vector.push(Coordinates {
                    letter: *current_letter,
                    number: row_number,
                });
            }
        }
        return_vector
    }

    pub fn legal_move_available(&self) -> bool {
        let mut legal_move_available = false;
        for coordinate in Self::board_coords() {
            match self.retreive_square(&coordinate) {
                Ok(square) => {
                    if !square.get_legal_targets(&coordinate, self).is_empty() {
                        // if it's not empty we know that there are legal targets that would not put the king in danger, so we can break early. We only need one legal move to be available
                        legal_move_available = true;
                        break;
                    }
                }
                Err(_) => {
                    panic!("Error. Attempted to find move legality for square {}, and it did not exist", coordinate);
                }
            }
        }

        legal_move_available
    }

    pub fn show_me_legal_squares(&self, coords: &Coordinates) {
        match self.retreive_square(coords) {
            Ok(retrieved_square) => {
                retrieved_square.show_me_legal_squares(coords, self);
            }
            Err(_) => {}
        }
    }

    fn convert_row_usize(size: usize) -> Result<usize, &'static str> {
        if size > 0 && size < 9 {
            Ok(7 - (size - 1))
        } else {
            Err("attempted to perform subtraction out of bounds of usize")
        }
    }

    pub fn retreive_square(&self, coords: &Coordinates) -> Result<Square, &'static str> {
        match board_safe_isize_converter(coords.number) {
            Ok(usize_number) => match board_safe_isize_converter(coords.letter.eval()) {
                Ok(usize_letter) => match Self::convert_row_usize(usize_number) {
                    Ok(converted_size) => {
                        Ok((self.rows[converted_size].squares[usize_letter]).clone())
                    }
                    Err(error_text) => Err(error_text),
                },
                Err(letter_text) => Err(letter_text),
            },
            Err(number_text) => Err(number_text),
        }
    }

    pub fn set_square(&mut self, coords: &Coordinates, square: Square) {
        match board_safe_isize_converter(coords.number) {
            Ok(usize_number) => match board_safe_isize_converter(coords.letter.eval()) {
                Ok(usize_letter) => match Self::convert_row_usize(usize_number) {
                    Ok(converted_rowsize) => {
                        self.rows[converted_rowsize].squares[usize_letter] = square;
                    }
                    Err(_) => {}
                },
                Err(_) => (),
            },
            Err(_) => (),
        }
    }

    pub fn king_can_castle(&self, king_color: PieceColor, is_kingside_query: bool) -> bool {
        if self.is_king_in_danger(king_color) {
            //now now, no castling out of check.
            return false;
        }
        match king_color {
            PieceColor::Black => {
                self.black_side_information
                    .king_can_castle(is_kingside_query)
                    && self.is_castling_safe(king_color, is_kingside_query)
            }
            PieceColor::White => {
                self.white_side_information
                    .king_can_castle(is_kingside_query)
                    && self.is_castling_safe(king_color, is_kingside_query)
            }
        }
    }

    fn is_castling_safe(&self, king_color: PieceColor, is_kingside_query: bool) -> bool {
        //logical shorting means that we can safely blind check.
        let mut copied_board = self.clone();

        let row_number: isize = match king_color {
            PieceColor::Black => 8,
            PieceColor::White => 1,
        };

        let empty_square = Square::Empty;
        match copied_board.retreive_square(&Coordinates {
            letter: ColumnLetter::E,
            number: row_number,
        }) {
            Ok(found_square) => {
                let moved_square = found_square.clone();
                if is_kingside_query {
                    copied_board.set_square(
                        &Coordinates {
                            letter: ColumnLetter::E,
                            number: row_number,
                        },
                        empty_square,
                    );
                    copied_board.set_square(
                        &Coordinates {
                            letter: ColumnLetter::F,
                            number: row_number,
                        },
                        moved_square,
                    );
                    if copied_board.is_king_in_danger(king_color) {
                        //keep your functions flat and return early.
                        return false;
                    }
                    //ok, so we know that it's safe to keep going. Lets check the next state.
                    copied_board.set_square(
                        &Coordinates {
                            letter: ColumnLetter::F,
                            number: row_number,
                        },
                        empty_square,
                    );
                    copied_board.set_square(
                        &Coordinates {
                            letter: ColumnLetter::G,
                            number: row_number,
                        },
                        moved_square,
                    );
                    if copied_board.is_king_in_danger(king_color) {
                        return false;
                    }
                    // we don't need to check hopping the rook over the king, because there isn't a way to threaten the king by moving that rook.
                } else {
                    copied_board.set_square(
                        &Coordinates {
                            letter: ColumnLetter::E,
                            number: row_number,
                        },
                        empty_square,
                    );
                    copied_board.set_square(
                        &Coordinates {
                            letter: ColumnLetter::D,
                            number: row_number,
                        },
                        moved_square,
                    );
                    if copied_board.is_king_in_danger(king_color) {
                        //keep your functions flat and return early.
                        return false;
                    }
                    //ok, so we know that it's safe to keep going. Lets check the next state.
                    copied_board.set_square(
                        &Coordinates {
                            letter: ColumnLetter::D,
                            number: row_number,
                        },
                        empty_square,
                    );
                    copied_board.set_square(
                        &Coordinates {
                            letter: ColumnLetter::C,
                            number: row_number,
                        },
                        moved_square,
                    );
                    if copied_board.is_king_in_danger(king_color) {
                        return false;
                    }
                }

                //if we made it here, then it's safe to return true.
                true
            }
            Err(_) => {
                panic!(
                    "You failed to retreive the square the king should be on at E{}",
                    row_number
                )
            }
        }
    }

    pub fn remove_castling_rights(&mut self, side_color: PieceColor, is_kingside: bool) {
        match side_color {
            PieceColor::Black => self
                .black_side_information
                .remove_castling_rights(is_kingside),
            PieceColor::White => self
                .white_side_information
                .remove_castling_rights(is_kingside),
        }
    }

    pub fn is_king_in_danger(&self, king_color: PieceColor) -> bool {
        let target_king_coordinates = match king_color {
            PieceColor::Black => self.black_side_information.current_king_square,
            PieceColor::White => self.white_side_information.current_king_square,
        };

        for letter in ColumnLetter::iterator() {
            for number in 1..9 {
                let from_coords = Coordinates {
                    letter: *letter,
                    number: number,
                };
                match self.retreive_square(&from_coords) {
                    Ok(gotten_square) => {
                        match gotten_square {
                            Square::Full(piece) => {
                                // we only care about the ability of other pieces to tkae our king;
                                if piece.color != king_color {
                                    //only pieces of the opposite color can threaten the king
                                    if self.square_threatens_square(
                                        &from_coords,
                                        &target_king_coordinates,
                                    ) {
                                        // oops, we found a square that threatens the king. Can't allow that!
                                        return true;
                                    }
                                }
                            }
                            Square::Empty => (),
                        }
                    }
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
                        let (legal, _, _, _, _, _, _, _) = parse_move_legality(from, to, self);
                        if legal {
                            match piece.kind {
                                PieceKind::Pawn => {
                                    //pawns are a special case because they can move up or down, but can only take on the diagonal.
                                    distance_information.distance == 2
                                        && (distance_information.move_direction
                                            != MoveDirection::Up
                                            && distance_information.move_direction
                                                != MoveDirection::Down)
                                }
                                _ => true,
                            }
                        } else {
                            false
                        }
                    }
                    Square::Empty => false, //can't threaten another square with an empty square.
                }
            }
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
                    }
                    Square::Empty => true, //Expected and fine
                }
            }
            Err(_) => {
                Self::panic_in_twixt_hither();
                false
            }
        }
    }

    pub fn twixt_hither_and_yon(
        &self,
        from: &Coordinates,
        to: &Coordinates,
        direction: MoveDirection,
    ) -> bool {
        let mut path_clear = true;

        let from_letter_value = from.letter.eval();
        let to_letter_value = to.letter.eval();
        let from_number_value = from.number;
        let to_number_value = to.number;
        match direction {
            MoveDirection::Down => {
                for i in (to_number_value + 1)..(from_number_value) {
                    if !self.inner_path_clear_checking(
                        &mut path_clear,
                        &Coordinates {
                            letter: from.letter,
                            number: i,
                        },
                    ) {
                        break;
                    }
                }
            }
            MoveDirection::Up => {
                for i in (from_number_value + 1)..to_number_value {
                    if !self.inner_path_clear_checking(
                        &mut path_clear,
                        &Coordinates {
                            letter: from.letter,
                            number: i,
                        },
                    ) {
                        break;
                    }
                }
            }
            MoveDirection::Left => {
                for i in (to_letter_value + 1)..from_letter_value {
                    match ColumnLetter::construct_letter_from_isize(i) {
                        Ok(found_letter) => {
                            if !self.inner_path_clear_checking(
                                &mut path_clear,
                                &Coordinates {
                                    letter: found_letter,
                                    number: from_number_value,
                                },
                            ) {
                                break;
                            }
                        }
                        Err(_) => {
                            panic!("attempted to access out of bounds");
                        }
                    }
                }
            }
            MoveDirection::Right => {
                for i in (from_letter_value + 1)..to_letter_value {
                    match ColumnLetter::construct_letter_from_isize(i) {
                        Ok(found_letter) => {
                            if !self.inner_path_clear_checking(
                                &mut path_clear,
                                &Coordinates {
                                    letter: found_letter,
                                    number: from_number_value,
                                },
                            ) {
                                break;
                            }
                        }
                        Err(_) => {
                            panic!("attempted to acces out of bounds");
                        }
                    }
                }
            }
            MoveDirection::Diagonal(DiagonalDirection::DownLeft) => {
                let distance = from_letter_value - to_letter_value;
                for i in 1..distance {
                    match ColumnLetter::construct_letter_from_isize(from_letter_value - i) {
                        Ok(found_letter) => {
                            if self.inner_path_clear_checking(
                                &mut path_clear,
                                &Coordinates {
                                    letter: found_letter,
                                    number: from_number_value - i,
                                },
                            ) {
                                break;
                            }
                        }
                        Err(_) => {
                            panic!("attempted to access out of bounds")
                        }
                    }
                }
            }
            MoveDirection::Diagonal(DiagonalDirection::DownRight) => {
                let distance = to_letter_value - from_letter_value;
                for i in 1..distance {
                    match ColumnLetter::construct_letter_from_isize(from_letter_value + i) {
                        Ok(found_letter) => {
                            if !self.inner_path_clear_checking(
                                &mut path_clear,
                                &Coordinates {
                                    letter: found_letter,
                                    number: from_number_value - i,
                                },
                            ) {
                                break;
                            }
                        }
                        Err(_) => {
                            panic!("attempted to access out of bounds")
                        }
                    }
                }
            }
            MoveDirection::Diagonal(DiagonalDirection::UpLeft) => {
                let distance = to_number_value - from_number_value;
                for i in 1..distance {
                    match ColumnLetter::construct_letter_from_isize(from_letter_value - i) {
                        Ok(found_letter) => {
                            if !self.inner_path_clear_checking(
                                &mut path_clear,
                                &Coordinates {
                                    letter: found_letter,
                                    number: from_number_value + i,
                                },
                            ) {
                                break;
                            }
                        }
                        Err(_) => {
                            panic!("attempted to access out of bounds")
                        }
                    }
                }
            }
            MoveDirection::Diagonal(DiagonalDirection::UpRight) => {
                let distance = to_letter_value - from_letter_value;
                for i in 1..distance {
                    // we can rely on the distance being equal, otherwise the move is illegal.
                    match ColumnLetter::construct_letter_from_isize(from_letter_value + i) {
                        Ok(found_letter) => {
                            if !self.inner_path_clear_checking(
                                &mut path_clear,
                                &Coordinates {
                                    letter: found_letter,
                                    number: from_number_value + i,
                                },
                            ) {
                                break;
                            }
                        }
                        Err(_) => {
                            panic!("attempted to access out of bounds")
                        }
                    }
                }
            }
            _ => (), //if we are here, the move is either a JHook, which is allowed to jump, or illegal,
        }

        path_clear
    }

    fn update_king_location(&mut self, coords: Coordinates, king_color: PieceColor) {
        match king_color {
            PieceColor::Black => self
                .black_side_information
                .update_king_location(&coords.letter, &coords.number),
            PieceColor::White => self
                .white_side_information
                .update_king_location(&coords.letter, &coords.number),
        }
    }

    fn castle(&mut self, king_color: PieceColor, is_kingside: bool) {
        let row_number = match king_color {
            PieceColor::White => 1,
            PieceColor::Black => 8,
        };
        let opt_pulled_king = self
            .retreive_square(&Coordinates {
                letter: ColumnLetter::E,
                number: row_number,
            })
            .clone();
        let opt_pulled_rook = match is_kingside {
            true => self
                .retreive_square(&Coordinates {
                    letter: ColumnLetter::H,
                    number: row_number,
                })
                .clone(),
            false => self
                .retreive_square(&Coordinates {
                    letter: ColumnLetter::A,
                    number: row_number,
                })
                .clone(),
        };
        let empty_square = Square::Empty;

        match opt_pulled_king {
            Ok(pulled_king) => match opt_pulled_rook {
                Ok(pulled_rook) => {
                    match is_kingside {
                        true => {
                            self.set_square(
                                &Coordinates {
                                    letter: ColumnLetter::H,
                                    number: row_number,
                                },
                                empty_square,
                            );
                            self.set_square(
                                &Coordinates {
                                    letter: ColumnLetter::E,
                                    number: row_number,
                                },
                                empty_square,
                            );
                            self.set_square(
                                &Coordinates {
                                    letter: ColumnLetter::G,
                                    number: row_number,
                                },
                                pulled_king,
                            );
                            self.set_square(
                                &Coordinates {
                                    letter: ColumnLetter::F,
                                    number: row_number,
                                },
                                pulled_rook,
                            );
                        }
                        false => {
                            self.set_square(
                                &Coordinates {
                                    letter: ColumnLetter::A,
                                    number: row_number,
                                },
                                empty_square,
                            );
                            self.set_square(
                                &Coordinates {
                                    letter: ColumnLetter::E,
                                    number: row_number,
                                },
                                empty_square,
                            );
                            self.set_square(
                                &Coordinates {
                                    letter: ColumnLetter::C,
                                    number: row_number,
                                },
                                pulled_king,
                            );
                            self.set_square(
                                &Coordinates {
                                    letter: ColumnLetter::D,
                                    number: row_number,
                                },
                                pulled_rook,
                            );
                        }
                    };
                }
                Err(_) => {
                    panic!("attempted to castle, but could not retreive rook")
                }
            },
            Err(_) => {
                panic!("attempted to castle, but could not retreive king")
            }
        }
    }

    pub fn move_piece(&mut self, from: &Coordinates, to: &Coordinates) -> MoveResult {
        let mut move_result: MoveResult;
        let opt_from_square = self.retreive_square(&from);
        match opt_from_square {
            Ok(from_square) => {
                let replacement_square = from_square.clone();
                match from_square {
                    Square::Full(piece) => {
                        if piece.color == self.turn {
                            let (
                                move_legal,
                                taking_piece,
                                target_piece_color,
                                target_piece_kind,
                                move_direction,
                                move_distance,
                                opt_passant_target,
                                opt_new_passant_legal,
                            ) = parse_move_legality(from, to, self);

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
                                    }
                                    PieceKind::Rook => {
                                        // we need to check if they're moving off their original square, and negate castling rights as necessary.
                                        if from.letter == ColumnLetter::A {
                                            //we can just do this dumbly, since it doesn't cost much, and firing it off every time ensures safety.
                                            //Lesson: sometimes it's cheaper to just ensure bool state than it is to check every time.
                                            self.remove_castling_rights(piece.color, false);
                                        } else if from.letter == ColumnLetter::H {
                                            self.remove_castling_rights(piece.color, true);
                                        }
                                    }
                                    _ => {}
                                }
                                self.opt_legal_passant_square = opt_new_passant_legal;
                                self.set_square(&from, Square::Empty);
                                self.set_square(&to, replacement_square);
                                match opt_passant_target {
                                    Some(passant_target) => {
                                        self.set_square(&passant_target, Square::Empty);
                                    }
                                    None => {}
                                }
                                if taking_piece {
                                    self.add_piece_to_kills(target_piece_kind, target_piece_color);
                                }
                                if taking_piece || piece.kind == PieceKind::Pawn {
                                    self.half_turns = 0;
                                } else {
                                    self.half_turns += 1;
                                }
                                match self.turn {
                                    PieceColor::Black => {
                                        self.turn = PieceColor::White;
                                        self.full_turns += 1;
                                    }
                                    PieceColor::White => self.turn = PieceColor::Black,
                                }
                                let opponent_color = match piece.color {
                                    PieceColor::Black => PieceColor::White,
                                    PieceColor::White => PieceColor::Black,
                                };
                                //we can make the move they are requesting. Lets check what state this leaves the board in.
                                match king_checkmate_state(opponent_color, &self) {
                                    MateState::Check => match opponent_color {
                                        PieceColor::Black => {
                                            move_result = MoveResult::BlackKingChecked;
                                        }
                                        PieceColor::White => {
                                            move_result = MoveResult::WhiteKingChecked;
                                        }
                                    },
                                    MateState::CheckMate => match opponent_color {
                                        PieceColor::Black => {
                                            move_result = MoveResult::BlackKingCheckmated
                                        }
                                        PieceColor::White => {
                                            move_result = MoveResult::WhiteKingCheckmated
                                        }
                                    },
                                    MateState::StaleMate => {
                                        move_result = MoveResult::Stalemate;
                                    }
                                    MateState::Safe => {
                                        move_result = MoveResult::CompletedSafely;
                                    }
                                }
                            } else {
                                move_result = MoveResult::MoveIllegal;
                            }
                        } else {
                            move_result = MoveResult::WrongTurn;
                        }
                    } //
                    Square::Empty => {
                        move_result = MoveResult::EmptySquare;
                    }
                }
            }
            Err(_) => {
                move_result = MoveResult::MoveIllegal;
            }
        }

        if self.half_turns == 100 {
            move_result = MoveResult::Stalemate;
        }

        move_result
    }

    pub fn add_piece_to_kills(&mut self, piece_kind: PieceKind, piece_color: PieceColor) {
        match piece_color {
            PieceColor::Black => self.white_side_information.add_taken_piece(piece_kind),
            PieceColor::White => self.black_side_information.add_taken_piece(piece_kind),
        }
    }

    pub fn adjust_taken_pieces(&mut self) {
        let mut black_pawn_count = 0;
        let mut black_rook_count = 0;
        let mut black_knight_count = 0;
        let mut black_bishop_count = 0;
        let mut black_queen_count = 0;
        let mut black_king_count = 0;

        let mut white_pawn_count = 0;
        let mut white_rook_count = 0;
        let mut white_knight_count = 0;
        let mut white_bishop_count = 0;
        let mut white_queen_count = 0;
        let mut white_king_count = 0;

        for row in self.rows.iter() {
            for square in row.squares.iter() {
                match square {
                    Square::Full(piece) => match piece.kind {
                        PieceKind::Pawn => {
                            if piece.color == PieceColor::Black {
                                black_pawn_count += 1;
                            } else {
                                white_pawn_count += 1;
                            }
                        }
                        PieceKind::Rook => {
                            if piece.color == PieceColor::Black {
                                black_rook_count += 1;
                            } else {
                                white_rook_count += 1;
                            }
                        }
                        PieceKind::Knight => {
                            if piece.color == PieceColor::Black {
                                black_knight_count += 1;
                            } else {
                                white_knight_count += 1;
                            }
                        }
                        PieceKind::Bishop => {
                            if piece.color == PieceColor::Black {
                                black_bishop_count += 1;
                            } else {
                                white_bishop_count += 1;
                            }
                        }
                        PieceKind::Queen => {
                            if piece.color == PieceColor::Black {
                                black_queen_count += 1;
                            } else {
                                white_queen_count += 1;
                            }
                        }
                        PieceKind::King => {
                            if piece.color == PieceColor::Black {
                                black_king_count += 1;
                            } else {
                                white_king_count += 1;
                            }
                        }
                    },
                    Square::Empty => {}
                }
            }
        }

        if white_king_count < 1 || black_king_count < 1 {
            panic!("Oops! It looks like you don't have a king on the board! How do you expect to win the game???");
        }

        //now lets adjust to taken counts by reduction
        black_pawn_count = 8 - black_pawn_count;
        white_pawn_count = 8 - white_pawn_count;

        black_rook_count = 2 - black_rook_count;
        white_rook_count = 2 - white_rook_count;

        black_knight_count = 2 - black_knight_count;
        white_knight_count = 2 - white_knight_count;

        black_bishop_count = 2 - black_bishop_count;
        white_bishop_count = 2 - white_bishop_count;

        black_queen_count = 1 - black_queen_count;
        white_queen_count = 1 - white_queen_count;

        self.white_side_information.adjust_taken_pieces(
            black_pawn_count,
            black_rook_count,
            black_knight_count,
            black_bishop_count,
            black_queen_count,
        );
        self.black_side_information.adjust_taken_pieces(
            white_pawn_count,
            white_rook_count,
            white_knight_count,
            white_bishop_count,
            white_queen_count,
        );
    }

    pub fn show_taken_pieces(&self, color: PieceColor) {
        print!("\n< ");
        match color {
            PieceColor::Black => {
                for piece in self.black_side_information.taken_pieces.iter() {
                    print!("{} ", piece);
                }
            }
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
            println!("[{} ]{}", i, row);
            i -= 1;
            let sleep_time = time::Duration::from_millis(150);
            thread::sleep(sleep_time);
        }
        print!("    ");
        for letter in ColumnLetter::iterator() {
            print!("[{} ]", letter)
        }
        self.show_taken_pieces(PieceColor::White);
        if !black_winning && !score_equal {
            println!("+{}", white_score - black_score);
        }
        println!("");
        Ok(())
    }
}
