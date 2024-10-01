use core::fmt;
use std::fmt::write;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black
}


impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::White => write!(f, "w"),
            Self::Black => write!(f, "b")
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

impl fmt::Display for PieceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pawn => write!(f, "P"),
            Self::Rook => write!(f, "R"),
            Self::Knight => write!(f, "N"),
            Self::Bishop => write!(f, "B"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K")
        }
    }
}

impl PieceKind {
    pub fn get_range(&self) -> u32 {
        match self {
            PieceKind::Pawn | PieceKind::King => 1,
            PieceKind::Knight => 2,
            _ => 8
        }
    }

    pub fn get_value(&self) -> u32 {
        match self {
            PieceKind::Pawn => 1,
            PieceKind::Knight | PieceKind::Bishop => 3,
            PieceKind::Rook => 4,
            PieceKind::Queen => 8,
            PieceKind::King => 0
        }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print!("{}{}", self.color, self.kind);
        Ok(())
    }
}