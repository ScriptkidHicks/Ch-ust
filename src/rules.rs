use crate::board::*;
use crate::pieces::*;

pub enum Mate_State {
    StaleMate,
    CheckMate,
    Check,
    Safe
}

pub fn king_checkmate_state(king_color: PieceColor, chess_board: &Board) -> Mate_State {
    let king_currently_in_danger = chess_board.is_king_in_danger(king_color);
    let legal_move_available = chess_board.legal_move_available(king_color);

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

    match board.retreive_square(&from) {
        Ok(from_square) => {
            match from_square {
                Square::Full(piece) => {
                    copied_board.set_square(&from, Square::Empty);
                    copied_board.set_square(&to, from_square.clone());
                    copied_board.is_king_in_danger(piece.color)
                },
                Square::Empty => panic!("You tried to move out of an empty square in would_king_be_in_danger!")
            }
        },
        Err(_) => {
            panic!("You tried to move a piece to an invalid location in would_king_be_in_danger")
        }
    }        
}

pub fn parse_move_legality(from: &Coordinates, to: &Coordinates, chess_board: &Board) -> (bool, bool, PieceColor, PieceKind, MoveDirection, isize) {
    let opt_from_square = chess_board.retreive_square(&from);
    let opt_to_square = chess_board.retreive_square(&to);
    let move_information: SquareToSquareInformation = measure_distance(from, to);
    let mut successful = false;
    let mut target_square_piece_color: PieceColor = PieceColor::Black;
    let mut target_square_piece_kind: PieceKind = PieceKind::Pawn;
    let mut taking_piece = false;
    let mut color_legal = false;

    //we can skip a lot of work by just checking that from and to are valid locations
    match opt_from_square {
        Ok(from_square) => {
            match opt_to_square {
                Ok(to_square) => {
                    let no_interference = chess_board.twixt_hither_and_yon(from, to, move_information.move_direction);
                    if no_interference {
                        match from_square {
                            Square::Full(from_piece) => {
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
                                    match from_piece.kind {
                                        PieceKind::Pawn => {
                                            if taking_piece {
                                                match from_piece.color {
                                                    PieceColor::Black => if move_information.distance == 2 && move_information.move_direction == MoveDirection::DownLeft || move_information.move_direction == MoveDirection::DownRight {successful = true;},
                                                    PieceColor::White => if move_information.distance == 2 && move_information.move_direction == MoveDirection::UpLeft || move_information.move_direction == MoveDirection::UpRight {
                                                        successful = true;
                                                    },
                                                }
                                            } else {
                                                if move_information.distance == 1 {
                                                    match from_piece.color {
                                                        PieceColor::Black => if move_information.move_direction == MoveDirection::Down {successful = true;},
                                                        PieceColor::White => if move_information.move_direction == MoveDirection::Up {successful = true;},
                                                    }
                                                } else if move_information.distance == 2 {
                                                    if (move_information.move_direction == MoveDirection::Up || move_information.move_direction == MoveDirection::Down) {
                                                            match from_piece.color {
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
                                                MoveDirection::Up | MoveDirection::Down => (),
                                                _ => {
                                                    //for any of the other diagonal moves we allow this move distance
                                                    successful = true;
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
                },
                Err(_) => {
                    // again we can just rely on this being illegal
                }
            }
        },
        Err(_) => {
            //we can safely do nothing, and let the illegal return handle it
        }
    }

    

    (successful, taking_piece, target_square_piece_color, target_square_piece_kind, move_information.move_direction, move_information.distance)
}