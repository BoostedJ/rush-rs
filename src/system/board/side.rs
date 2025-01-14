use super::Color;

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