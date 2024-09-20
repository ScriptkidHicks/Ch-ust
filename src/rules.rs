use crate::board::*;
use crate::pieces::*;

pub fn parse_move_legality(kind: PieceKind, from: Coordinates, to: Coordinates, chess_board: Board) -> bool {
    match kind {
        PieceKind::Pawn => true,
        PieceKind::Knight => true,
        PieceKind::Bishop => true,
        PieceKind::Rook => true,
        PieceKind::King => true,
        PieceKind::Queen => true
    }
}