use crate::board::*;
use crate::pieces::*;

pub enum Mate_State {
    StaleMate,
    CheckMate,
    Check,
    Safe
}

pub fn king_checkmate_state(king_color: PieceColor, chess_board: &Board, opt_previous_turn_board: Option<&Board>) -> Mate_State {
    let king_currently_in_danger = chess_board.is_king_in_danger(king_color);
    let legal_move_available = chess_board.legal_move_available(king_color, opt_previous_turn_board);

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

pub fn square_meets_expectations(
    board: &Board,
    target_coords: &Coordinates,
    should_be_empty: bool,
    expected_piece_color: PieceColor,
    expected_piece_kind: PieceKind
) -> bool {
    let mut meets_expectations = false;

    match board.retreive_square(target_coords) {
        Ok(square) => {
            match square {
                Square::Full(piece) => {
                    meets_expectations = !should_be_empty && (piece.color == expected_piece_color) && (piece.kind == expected_piece_kind);
                },
                Square::Empty => {
                    meets_expectations = should_be_empty;
                }
            }
        },
        Err(_) => {
            panic!("Tried to retreive nonexistant square in square_meets_expectations")
        }
    }

    meets_expectations
}

pub fn en_passant_legal( 
    target_pawn_color: PieceColor, 
    from: &Coordinates, 
    to: &Coordinates, 
    previous_turn_board: &Board,
    current_turn_board: &Board) 
    -> bool 
{
    //if we're here we can safely assume that the move they're trying to do is en passant.
    let mut move_legal = false;
    //lets triangulate the target square.
    let current_turn_coords = Coordinates {letter: to.letter, number: from.number };
    let previous_turn_coords = match target_pawn_color {
        PieceColor::White => {
            Coordinates {letter: to.letter, number: current_turn_coords.number - 2}
        },
        PieceColor::Black => {
            Coordinates {letter: to.letter, number: current_turn_coords.number + 2}
        }
    };

    //for this to be legal, 3 things must be true.
    //1. there must be a pawn (of the opposite color) in the target square
    //2. that square must have been empty the previous turn.
    //3. that pawn must have moved from 2 squares away the previous turn; ie: the square should then have been empty,
    //   and the square "above" it must have been empty. Since we legality check all other moves, there's no other way 
    //   to get a pawn into that position

    let current_target_square_as_expected = square_meets_expectations(current_turn_board, &current_turn_coords, false, target_pawn_color, PieceKind::Pawn);

    if (current_target_square_as_expected) {
        let current_left_square_as_expected = square_meets_expectations(current_turn_board, &previous_turn_coords, true, target_pawn_color, PieceKind::Pawn);
        if (current_left_square_as_expected) {
            let previous_turn_left_square_meets_expectations = square_meets_expectations(previous_turn_board, &previous_turn_coords, false, target_pawn_color, PieceKind::Pawn);
            if (previous_turn_left_square_meets_expectations) {
                move_legal = true;
            }
        }
    }

    move_legal
}

pub fn parse_move_legality(from: &Coordinates, to: &Coordinates, chess_board: &Board, opt_previous_turn_board: Option<&Board>) -> (bool, bool, PieceColor, PieceKind, MoveDirection, isize, Option<Coordinates>) {
    let opt_from_square = chess_board.retreive_square(&from);
    let opt_to_square = chess_board.retreive_square(&to);
    let move_information: SquareToSquareInformation = measure_distance(from, to);
    let mut successful = false;
    let mut target_square_piece_color: PieceColor = PieceColor::Black;
    let mut target_square_piece_kind: PieceKind = PieceKind::Pawn;
    let mut taking_piece = false;
    let mut color_legal = false;
    let mut opt_passant_removal = None;

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
                                                    match move_information.move_direction {
                                                        MoveDirection::Down | MoveDirection::Up => {
                                                            match from_piece.color {
                                                                PieceColor::Black => {
                                                                    successful = from.number == 7;
                                                                },
                                                                PieceColor::White => {
                                                                    successful = from.number == 2;
                                                                }
                                                            }
                                                        },
                                                        MoveDirection::DownLeft | MoveDirection::DownRight | MoveDirection::UpLeft | MoveDirection::UpRight => {
                                                            match opt_previous_turn_board {
                                                                Some(previous_board) => {
                                                                    successful = en_passant_legal(from_piece.color.get_inverse_color(), from, to, previous_board, chess_board);
                                                                    if successful {
                                                                        taking_piece = true;
                                                                        target_square_piece_color = from_piece.color.get_inverse_color();
                                                                        target_square_piece_kind = PieceKind::Pawn;
                                                                        opt_passant_removal = Some(Coordinates {letter: to.letter, number: from.number});
                                                                    }
                                                                },
                                                                None => {}
                                                            }
                                                        },
                                                        _ => {}
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

    

    (successful, taking_piece, target_square_piece_color, target_square_piece_kind, move_information.move_direction, move_information.distance, opt_passant_removal)
}