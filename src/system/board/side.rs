use std::ops::Not;

#[derive(Debug, Clone, Copy)]
pub struct Side(pub Color);
impl Side {
    pub const WHITE: Side = Side(Color::White);
    pub const BLACK: Side = Side(Color::Black);
    
    pub fn piece_range(&self) -> std::ops::Range<usize> {
        match self.0 {
            Color::White => 0..6,
            Color::Black => 6..12,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Self;

    fn not(self)->Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}