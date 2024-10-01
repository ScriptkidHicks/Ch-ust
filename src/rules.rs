use crate::board::*;
use crate::pieces::*;

pub fn would_king_be_in_danger(board: Board, from: &Coordinates, to: &Coordinates) -> bool {
        let mut copied_board = board.clone();

        let from_square = board.retreive_square(&from);

        copied_board.set_square(&from, Square::Empty);
        copied_board.set_square(&to, from_square.clone());

        copied_board.is_king_in_danger()
    }

pub fn parse_move_legality(kind: PieceKind, from: &Coordinates, to: &Coordinates, chess_board: &Board) -> (bool, bool, PieceColor, PieceKind) {
    let move_information = measure_distance(from, to);
    let mut successful = false;
    let from_square = chess_board.retreive_square(&from);
    let to_square = chess_board.retreive_square(&to);
    let mut target_square_piece_color: PieceColor = PieceColor::Black;
    let mut target_square_piece_kind: PieceKind = PieceKind::Pawn;
    let from_square_piece: Piece;
    let mut taking_piece = false;
    let mut color_legal = false;

    let no_interference = chess_board.twixt_hither_and_yon(from, to, move_information.move_direction);

    if no_interference {
        match from_square {
            Square::Full(from_piece) => {
                from_square_piece = *from_piece;
                match to_square {
                    Square::Full(to_piece) => {
                        // I simply can't let you move your pieces into themselves.
                        if from_piece.color != to_piece.color {
                            color_legal = true;
                        }
                        target_square_piece_color = to_piece.color;
                        target_square_piece_kind = to_piece.kind;
                        taking_piece = true;
                    },
                    Square::Empty => color_legal = true
                };

                if color_legal {
                    match kind {
                        PieceKind::Pawn => {
                            if taking_piece {
                                match from_square_piece.color {
                                    PieceColor::Black => if move_information.distance == 2 && move_information.move_direction == MoveDirection::DownLeft || move_information.move_direction == MoveDirection::DownRight {successful = true;},
                                    PieceColor::White => if move_information.distance == 2 && move_information.move_direction == MoveDirection::UpLeft || move_information.move_direction == MoveDirection::UpRight {
                                        successful = true;
                                    },
                                }
                            } else {
                                if move_information.distance == 1 {
                                    match from_square_piece.color {
                                        PieceColor::Black => if move_information.move_direction == MoveDirection::Down {successful = true;},
                                        PieceColor::White => if move_information.move_direction == MoveDirection::Up {successful = true;},
                                    }
                                } else if move_information.distance == 2 {
                                    match from_square_piece.color {
                                    PieceColor::Black => {
                                        if from.number == 7 {
                                            successful = true;
                                        }
                                    },
                                    PieceColor::White => {
                                        if from.number == 2 {
                                            successful = true;
                                        }
                                    }
                                }
                            }
                        };
                    },
                    PieceKind::Knight => {
                        if move_information.move_direction == MoveDirection::JHook {
                            successful = true;
                        };
                    },
                    PieceKind::Rook => {
                        match move_information.move_direction {
                            MoveDirection::Down | MoveDirection::Up | MoveDirection::Left | MoveDirection::Right => successful = true,
                            _ => ()
                        };
                    },
                    PieceKind::Bishop => {
                        match move_information.move_direction {
                            MoveDirection::DownLeft | MoveDirection::DownRight | MoveDirection::UpLeft | MoveDirection::UpRight => successful = true,
                            _ => ()
                        };
                    },
                    PieceKind::King => {
                        if move_information.distance == 1 {
                            successful = true;
                        } else if move_information.distance == 2 {
                            match from_square_piece.color {
                                PieceColor::Black => {

                                },
                                PieceColor::White => {
                                    
                                }
                            }
                        }
                    },
                    PieceKind::Queen => {
                        match move_information.move_direction {
                            MoveDirection::IllegalMove | MoveDirection::JHook | MoveDirection::NoMove => (), //do nothing. Only moves the queen can't make
                            _ => {successful = true;}
                        }
                    }
                };
            } 
            }
            _ => ()
        };
    
        successful = successful && !would_king_be_in_danger(chess_board.clone(), from, to);
    }

    

    (successful, taking_piece, target_square_piece_color, target_square_piece_kind)
}