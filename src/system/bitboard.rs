use std::fmt;
use super::*;



#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn new() -> Self {
        BitBoard(0x0000_0000_0000_FF00)
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask: u16 = rank*8 + file;
                let bit = (self.0 >> mask) & 1;
                write!(f, "{} ", bit)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_display() {
        let bitboard = BitBoard::new();
        println!("{}", bitboard);
    }
}
