use crate::board::*;
use crate::pieces::*;

pub enum Mate_State {
    StaleMate,
    CheckMate,
    Check,
    Safe
}

fn legal_move_available_from_this_square(square: &Square, turn_color: PieceColor, chess_board: &Board) -> bool {
    match square {
        Square::Empty => {
            false
        }, 
        Square::Full(piece) => {
            if piece.color == turn_color {
                match piece.kind {
                    PieceKind::Pawn => {
                        match piece.color {
                            PieceColor::Black => {

                            },
                            PieceColor::White => {
                                
                            }
                        }
                        true
                    },
                    PieceKind::Rook => {
                        true
                    },
                    PieceKind::Knight => {
                        true
                    },
                    PieceKind::Bishop => {
                        true
                    }, 
                    PieceKind::King => {
                        true
                    },
                    PieceKind::Queen => {
                        true
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn king_checkmate_state(king_color: PieceColor, chess_board: &Board) -> Mate_State {
    let king_currently_in_danger = chess_board.is_king_in_danger(king_color);
    let legal_move_available = chess_board.search_squares(king_color, legal_move_available_from_this_square);

    match (king_currently_in_danger, legal_move_available) {
        (true, true) => {
            Mate_State::Check
        },
        (true, false) => {
            Mate_State::CheckMate
        },
        (false, true) => {
            Mate_State::Safe
        },
        (false, false) => {
            Mate_State::StaleMate
        }
    }
}

pub fn would_king_be_in_danger(board: Board, from: &Coordinates, to: &Coordinates) -> bool {
    let mut copied_board = board.clone();

    let from_square = board.retreive_square(&from);
    match from_square {
        &Square::Full(piece) => {
            copied_board.set_square(&from, Square::Empty);
            copied_board.set_square(&to, from_square.clone());

            copied_board.is_king_in_danger(piece.color)
        },
        &Square::Empty => panic!("You tried to move out of an empty square in would_king_be_in_danger!")
    }        
}

pub fn parse_move_legality(kind: PieceKind, from: &Coordinates, to: &Coordinates, chess_board: &Board) -> (bool, bool, PieceColor, PieceKind, MoveDirection, usize) {
    let move_information: SquareToSquareInformation = measure_distance(from, to);
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
                            match move_information.move_direction {
                                MoveDirection::Left => {
                                    if chess_board.king_can_castle(from_piece.color, false) {
                                        //ok, we now need to do an additional check to see if castling the king on this side would be safe.
                                        successful = true;
                                    }
                                },
                                MoveDirection::Right => {
                                    if chess_board.king_can_castle(from_piece.color, true) {
                                        successful = true;
                                    }
                                },
                                _ => ()
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

    

    (successful, taking_piece, target_square_piece_color, target_square_piece_kind, move_information.move_direction, move_information.distance)
}