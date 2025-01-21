//! Bitboard operations and utilities
//! 
//! Board representation using 64-bit integer,
//! where each bit represents a square's occupancy
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn new() -> Self {
        /*
        00000000_00000000_00000000_00000000_00000000_00000000_{0}000000{0}_0000000{0}
                                                               ^h2      ^a2        ^a1
         */
        BitBoard(0x00_00_00_00_00_00_00_00)
    }

    pub fn set_bit(&mut self, square: usize) {
        self.0 |= 1u64 << square;
    }

    pub fn clear_bit(&mut self, square: usize) {
        self.0 &= !(1u64 << square);
    }

    pub fn get_bit(&mut self, square: usize) -> bool {
        (self.0 >> square) & 1 != 0
    }

    pub fn pop_count(&self) -> u32 {
        self.0.count_ones()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bitboard_operations() {
        let mut bb = BitBoard::new();

        bb.set_bit(0);
        assert_eq!(bb.get_bit(0), true);

        bb.clear_bit(0);
        assert_eq!(bb.get_bit(0), false);

        bb.set_bit(63);
        bb.set_bit(27);
        assert_eq!(bb.pop_count(), 2);
    } // SUCCESS
}