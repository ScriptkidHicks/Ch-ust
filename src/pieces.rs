use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn get_inverse_color(&self) -> PieceColor {
        match self {
            PieceColor::Black => PieceColor::White,
            PieceColor::White => PieceColor::Black,
        }
    }
}

impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::White => write!(f, "w"),
            Self::Black => write!(f, "b"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl fmt::Display for PieceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pawn => write!(f, "P"),
            Self::Rook => write!(f, "R"),
            Self::Knight => write!(f, "N"),
            Self::Bishop => write!(f, "B"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
        }
    }
}

impl PieceKind {
    pub fn get_value(&self) -> u32 {
        match self {
            PieceKind::Pawn => 1,
            PieceKind::Knight | PieceKind::Bishop => 3,
            PieceKind::Rook => 4,
            PieceKind::Queen => 8,
            PieceKind::King => 0,
        }
    }

    pub fn get_fen_string(&self) -> String {
        match self {
            PieceKind::Rook => "r".to_string(),
            PieceKind::Knight => "n".to_string(),
            PieceKind::Bishop => "b".to_string(),
            PieceKind::King => "k".to_string(),
            PieceKind::Queen => "q".to_string(),
            PieceKind::Pawn => "p".to_string(),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
}

impl fmt::Display for Piece {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        print!("{}{}", self.color, self.kind);
        Ok(())
    }
}

impl Piece {
    pub fn get_fen_string(&self) -> String {
        match self.color {
            PieceColor::White => self.kind.get_fen_string().to_uppercase(),
            _ => self.kind.get_fen_string(),
        }
    }
}
