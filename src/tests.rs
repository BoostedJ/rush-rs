#[cfg(test)]
mod tests {
    use crate::system::*;

    fn bitboards_to_board(piece_bitboards: [BitBoard; 12]) -> [[char; 8]; 8] {
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

    fn debug_print(bitboard: BitBoard) {
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
        let bitboard = BitBoard::new();
        debug_print(bitboard);
    } // SUCCESS

    #[test]
    fn debug_display_position() {
        let start_fen = "rnbqkbnr/pppppppp/8/4r3/2q212/1k111111/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed = parse_fen(start_fen);
        let piece_bitboards = parsed.piece_boards;
        let board = bitboards_to_board(piece_bitboards);
        
        // update if position is not starting position
        let expected = [
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
            ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', 'r', '.', '.', '.'],
            ['.', '.', 'q', '.', '.', '.', '.', '.'],
            ['.', 'k', '.', '.', '.', '.', '.', '.'],
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
        let parsed = parse_fen(start_fen);

        assert_eq!(parsed.piece_boards[0].pop_count(), 8); // 8 white pawns

        // game state
        assert_eq!(parsed.side_to_move, Color::White);
        assert_eq!(parsed.castling_rights.0, Castling::ANY_CASTLING);
        assert_eq!(parsed.en_passant, None);
        assert_eq!(parsed.half_move, 0);
        assert_eq!(parsed.full_move, 1);
    } // SUCCESS

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

    #[test]
    fn test_board_conversion() {
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed = parse_fen(start_fen);
        let mut board: Board = parsed.into();

        assert!(matches!(board.pieces[0], Some(Piece::WhiteRook)));
        assert!(matches!(board.pieces[63], Some(Piece::BlackRook)));

        board.castling_rights.empty();
        assert_eq!(board.castling_rights.0, Castling::NO_CASTLING);
        board.castling_rights.all();
        assert_eq!(board.castling_rights.0, Castling::ANY_CASTLING);
    } // SUCCESS

    #[test]
    fn test_board_supports() {
        use Color::White as white;
        use Color::Black as black;
        
        let fen = "rnbqkbnr/pppppppp/8/PPPPPPPP/pppppppp/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        let parsed = parse_fen(fen);
        let mut board: Board = parsed.into();

        board.reset();
        assert_eq!(board.get_side_to_move(), white);
        assert_eq!(board.occupancy(), 32);

        // {0: a1, 1: b1, 2: c1, 3: d1, 4: e1}
        assert_eq!(board.king_square(white).0, 4); // should be E1
        assert_eq!(board.king_square(black).0, 60); // E8

        assert!(board.has_bishop_pair(white));
        assert!(board.has_bishop_pair(black));

        let no_bishop_pair = "rn1qk1nr/pppppppp/8/8/8/8/PPPPPPPP/RN1QK1NR w KQkq - 0 1";
        let parsed = parse_fen(no_bishop_pair);
        let board: Board = parsed.into();
        assert!(!board.has_bishop_pair(white));
        assert!(!board.has_bishop_pair(black));
    } // SUCCESS

    #[test]
    fn test_board_indexing() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        let parsed = parse_fen(fen);
        let board: Board = parsed.into();

        let white = board.side_pieces(Side::WHITE);
        assert_eq!(white.pop_count(), 16);
        let white_knights = board.piece_bb(Side::WHITE, Piece::WhiteKnight);
        assert_eq!(white_knights.pop_count(), 2);
    } // SUCCESS
}