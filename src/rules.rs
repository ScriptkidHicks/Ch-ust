use crate::board::*;
use crate::pieces::*;

pub fn parse_move_legality(kind: PieceKind, from: &Coordinates, to: &Coordinates, chess_board: &Board) -> (bool, bool, PieceColor, PieceKind) {
    let move_information = measure_distance(from, to);
    let mut successful = false;
    let from_square = chess_board.retreive_square(&from);
    let to_square = chess_board.retreive_square(&to);
    let mut target_square_piece_color: PieceColor = PieceColor::Black;
    let mut target_square_piece_kind: PieceKind = PieceKind::Pawn;
    let mut taking_piece = false;
    let mut color_legal = false;

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
            }
        }
        _ => ()
    };
    if color_legal {
        match kind {
            PieceKind::Pawn => if taking_piece {
                 match move_information.move_direction {
                    // if taking a piece, pawns may only move diagonally
                     MoveDirection::Diagonal => {if move_information.distance == 1 {successful = true}},
                     _ => ()
                 }
            } else {
                match move_information.move_direction {
                    MoveDirection::Verticle => {if move_information.distance == 1 {successful = true}},
                    _ => ()
                }
            },
            PieceKind::Knight => 
                match move_information.move_direction {
                    MoveDirection::JHook => successful = true,
                    _ => ()
                },
            PieceKind::Bishop => 
                match move_information.move_direction {
                    MoveDirection::Diagonal => successful = true,
                    _ => ()
                },
            PieceKind::Rook => 
                match move_information.move_direction {
                    MoveDirection::Lateral | MoveDirection::Verticle => successful = true,
                    _ => ()
                },
            PieceKind::King => if move_information.distance == 1 {
                successful = true;
            },
            PieceKind::Queen => match  move_information.move_direction {
                MoveDirection::Diagonal | MoveDirection::Verticle | MoveDirection::Lateral => {
                    successful = true;
                },
                _ => ()
            }
        };
    } 
    

    (successful, taking_piece, target_square_piece_color, target_square_piece_kind)
}