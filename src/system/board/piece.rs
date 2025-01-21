//! Chess piece definitions and operations
//! 
//! Defines piece types, colors, and conversion utilities
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
    NONE,
}

impl Piece {
    pub fn to_index(&self) -> usize {
        match self {
            Piece::WhitePawn => 0,
            Piece::WhiteKnight => 1,
            Piece::WhiteBishop => 2,
            Piece::WhiteRook => 3,
            Piece::WhiteQueen => 4,
            Piece::WhiteKing => 5,
            Piece::BlackPawn => 6,
            Piece::BlackKnight => 7,
            Piece::BlackBishop => 8,
            Piece::BlackRook => 9,
            Piece::BlackQueen => 10,
            Piece::BlackKing => 11,
            Piece::NONE => 12,
        }
    }

    pub fn from_index(i: usize) -> Piece {
        match i {
            0 => Piece::WhitePawn,
            1 => Piece::WhiteKnight,
            2 => Piece::WhiteBishop,
            3 => Piece::WhiteRook,
            4 => Piece::WhiteQueen,
            5 => Piece::WhiteKing,
            6 => Piece::BlackPawn,
            7 => Piece::BlackKnight,
            8 => Piece::BlackBishop,
            9 => Piece::BlackRook,
            10 => Piece::BlackQueen,
            11 => Piece::BlackKing,
            _ => Piece::NONE,
        }
    }
}