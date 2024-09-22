use crate::board::*;
use crate::pieces::*;

pub fn parse_move_legality(kind: PieceKind, from: &Coordinates, to: &Coordinates, chess_board: &Board) -> bool {
    let move_information = measure_distance(from, to);
    let mut successful = false;
    let from_square = chess_board.retreive_square(&from);
    let to_square = chess_board.retreive_square(&to);
    let mut color_legal = false;

    match from_square {
        Square::Full(from_piece) => {
            match to_square {
                Square::Full(to_piece) => {
                    // I simply can't let you move your pieces into themselves.
                    if from_piece.color != to_piece.color {
                        color_legal = true;
                    }
                },
                Square::Empty => color_legal = true
            }
        }
        _ => ()
    };
    if color_legal {
        match kind {
            PieceKind::Pawn => (),
            PieceKind::Knight => 
            if 
                move_information.trueJHook
             {
                successful = true;
            },
            PieceKind::Bishop => (),
            PieceKind::Rook => (),
            PieceKind::King => (),
            PieceKind::Queen => ()
        };
    } 
    

    successful
}