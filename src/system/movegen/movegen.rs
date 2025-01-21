use super::{ Board, Square, Piece, BitBoard, Move };

pub struct MoveGenerator {
    // pre-calculated lookup tables
    pawn_attacks: [[BitBoard; 64]; 2], // sizes: BitBoard (8 bytes) * 64 (for each square) * 2 = 1KB total
    knight_moves: [BitBoard; 64], // 8 bytes * 64 = 512 bytes
    king_moves: [BitBoard; 64], // 512 bytes -- should be optimal for leaping pieces?
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    // move directions to make life simple later
    North = 8,
    South = -8,
    East = 1,
    West = -1,
    NorthEast = 9,
    NorthWest = 7,
    SouthEast = -7,
    SouthWest = -9,
}

impl MoveGenerator {
    // note: in binary, board is flipped
    const NOT_A_FILE: u64 = 0xfefefefefefefefe;  // 11111110
    const NOT_AB_FILE: u64 = 0xfcfcfcfcfcfcfcfc; // 11111100
    const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;  // 01111111
    const NOT_GH_FILE: u64 = 0x3f3f3f3f3f3f3f3f; // 00111111
    
    pub fn new() -> Self {
        Self {
            pawn_attacks: [[BitBoard(0); 64]; 2],
            knight_moves: [BitBoard(0); 64],
            king_moves: [BitBoard(0); 64],
        }
    }

    pub fn init_leaping_pieces(&mut self) {
        self.init_pawn_attacks();
        self.init_knight_moves();
        self.init_king_moves();
    }

    fn shift(bb: BitBoard, dir: Direction) -> BitBoard {
        let val = bb.0 as u64;
        match dir {
            Direction::North => BitBoard((val << 8) as u64),
            Direction::South => BitBoard((val >> 8) as u64),
            Direction::East => BitBoard(((val << 1) & Self::NOT_A_FILE as u64) as u64),
            Direction::West => BitBoard(((val >> 1) & Self::NOT_H_FILE as u64) as u64),
            Direction::NorthEast => BitBoard(((val << 9) & Self::NOT_A_FILE as u64) as u64),
            Direction::NorthWest => BitBoard(((val << 7) & Self::NOT_H_FILE as u64) as u64),
            Direction::SouthEast => BitBoard(((val >> 7) & Self::NOT_A_FILE as u64) as u64),
            Direction::SouthWest => BitBoard(((val >> 9) & Self::NOT_H_FILE as u64) as u64),
        }
    }

    fn init_pawn_attacks(&mut self) {
        for sq in 0..64 {
            let bb = BitBoard(1 << sq);

            // white pawns
            self.pawn_attacks[0][sq] = BitBoard(
                Self::shift(bb, Direction::NorthEast).0 |
                Self::shift(bb, Direction::NorthWest).0
            );

            // black pawns
            self.pawn_attacks[1][sq] = BitBoard(
                Self::shift(bb, Direction::SouthEast).0 |
                Self::shift(bb, Direction::SouthWest).0
            );
        }
    }

    fn init_knight_moves(&mut self) {
        for sq in 0..64 {
            let bb = BitBoard(1 << sq);
            let mut moves = BitBoard(0);

            // generate knight moves
            moves.0 |= ((bb.0 << 17) & Self::NOT_A_FILE) | // checks down2left1 
                        ((bb.0 << 15) & Self::NOT_H_FILE) |
                        ((bb.0 >> 17) & Self::NOT_H_FILE) |
                        ((bb.0 >> 15) & Self::NOT_A_FILE) |
                        ((bb.0 << 10) & Self::NOT_AB_FILE) |
                        ((bb.0 << 6) & Self::NOT_GH_FILE) |
                        ((bb.0 >> 10) & Self::NOT_GH_FILE) |
                        ((bb.0 >> 6) & Self::NOT_AB_FILE);
            self.knight_moves[sq] = moves;
        }
    }

    fn init_king_moves(&mut self) {
        for sq in 0..64 {
            let bb = BitBoard(1 << sq);
            let mut moves = BitBoard(0);
            self.king_moves[sq] = BitBoard(
                Self::shift(bb, Direction::NorthEast).0 |
                Self::shift(bb, Direction::North).0 |
                Self::shift(bb, Direction::NorthWest).0 |
                Self::shift(bb, Direction::West).0 |
                Self::shift(bb, Direction::East).0 |
                Self::shift(bb, Direction::SouthEast).0 |
                Self::shift(bb, Direction::South).0 |
                Self::shift(bb, Direction::SouthWest).0
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn debug_print(bb: BitBoard) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = (bb.0 >> square) & 1;
                print!("{} ", if bit == 1 { "1" } else { "." });
            }
            println!();
        }
        println!();
    }

    #[test]
    fn test_is_knight_move() {
        let mut mg = MoveGenerator::new();
        mg.init_leaping_pieces();

        let e4_moves = mg.knight_moves[28];
        let a1_moves = mg.knight_moves[0];
        let a2_moves = mg.knight_moves[1];

        assert_eq!(e4_moves.pop_count(), 8);
        assert_eq!(a1_moves.pop_count(), 2);
        assert_ne!(a2_moves.pop_count(), 2);

        println!("Knight on e4 moves:");
        debug_print(e4_moves);
        println!("Knight on a1 moves:");
        debug_print(a1_moves);
        println!("Knight on a2 moves:");
        debug_print(a2_moves);
    }

    #[test]
    fn test_is_king_move() {
        let mut mg = MoveGenerator::new();
        mg.init_leaping_pieces();

        let e4_moves = mg.king_moves[28];
        let a8_moves = mg.king_moves[63];

        assert_eq!(e4_moves.pop_count(), 8);
        assert_eq!(a8_moves.pop_count(), 3);

        println!("King on e4 moves:");
        debug_print(e4_moves);
        println!("King on a8 moves:");
        debug_print(a8_moves);
    }

    #[test]
    fn test_is_pawn_attack() {
        let mut mg = MoveGenerator::new();
        mg.init_leaping_pieces();

        let e2_white = mg.pawn_attacks[0][12];
        let e7_black = mg.pawn_attacks[1][52];
        assert_eq!(e2_white.pop_count(), 2);
        assert_eq!(e7_black.pop_count(), 2);

        let a2_white = mg.pawn_attacks[0][8];
        let h2_white = mg.pawn_attacks[0][15];
        assert_eq!(a2_white.pop_count(), 1);
        assert_eq!(h2_white.pop_count(), 1);

        println!("White pawn on e2 attacks:");
        debug_print(e2_white);
        println!("Black pawn on e7 attacks:");
        debug_print(e7_black);
        println!("White pawn on a2 attack:");
        debug_print(a2_white);
    }
}