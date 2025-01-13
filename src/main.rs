mod system;

fn main() {
    println!("Hello world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bitboards_to_board(piece_bitboards: [system::BitBoard; 12]) -> [[char; 8]; 8] {
        let mut board = [['.'; 8]; 8];
        
        for rank in 0..8 {
            for file in 0..8 {
                let square = rank * 8 + file;
                let mask = 1u64 << square;
                
                for (i, bb) in piece_bitboards.iter().enumerate() {
                    if bb.0 & mask != 0 {
                        board[7 - rank][file] = match i {
                            0 => 'P', 1 => 'N', 2 => 'B', 3 => 'R', 4 => 'Q', 5 => 'K',
                            6 => 'p', 7 => 'n', 8 => 'b', 9 => 'r', 10 => 'q', 11 => 'k',
                            _ => '.'
                        };
                    }
                }
            }
        }
        board
    }

    fn debug_print(bitboard: system::BitBoard) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let mask: u16 = rank*8 + file;
                let bit = (bitboard.0 >> mask) & 1;
                print!("{} ", bit);
            }
            println!();
        }
    }
    
    #[test]
    fn debug_bitboard_display() {
        let bitboard = system::BitBoard::new();
        debug_print(bitboard);
    } // SUCCESS

    #[test]
    fn debug_display_position() {
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed = system::parse_fen(start_fen);
        let piece_bitboards = parsed.piece_boards;
        let board = bitboards_to_board(piece_bitboards);
        
        // update if position is not starting position
        let expected = [
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
            ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
            ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
        ];

        assert_eq!(board, expected);

        // print board for visual verification
        for rank in &board {
            println!("{}", rank.iter().collect::<String>());
        }
    } // SUCCESS

    #[test]
    fn debug_fen_position_state() {
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed = system::parse_fen(start_fen);

        assert_eq!(parsed.side_to_move, system::Color::White);

        assert_eq!(parsed.castling_rights.0, system::Castling::ANY_CASTLING);

        assert_eq!(parsed.en_passant, None);

        assert_eq!(parsed.half_move, 0);
        assert_eq!(parsed.full_move, 1);
    } // SUCCESS
}