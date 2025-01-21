//! Board representation and manipulation
//! 
//! Board is a core data structure that maintains:
//! - Piece positions (both array and bitboard representation)
//! - Game state (castling rights, en passant, move counters)
//! - Current side to move
use super::{ BitBoard, Color, ParsedFen, Piece, Side, Square, Move };

pub struct Board {
    pub pieces: [Option<Piece>; 64],
    pub piece_boards: [BitBoard; 12],
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub side_to_move: Color,
    pub half_move: u8,
    pub full_move: u16,
}

impl From<ParsedFen> for Board {
    fn from(fen: ParsedFen) -> Self {
        let mut pieces = [None; 64];
        for (i, bb) in fen.piece_boards.iter().enumerate() {
            let mut board = bb.0;
            while board != 0 {
                let square = board.trailing_zeros() as usize;
                pieces[square] = Some(Piece::from_index(i));
                board &= board - 1;
            }
        }

        Self {
            pieces,
            piece_boards: fen.piece_boards,
            castling_rights: fen.castling_rights,
            en_passant: fen.en_passant,
            side_to_move: fen.side_to_move,
            half_move: fen.half_move,
            full_move: fen.full_move,
        }
    }
}

impl Board {
    pub fn make_move(&mut self, mv: Move) {
        match mv {
            Move::Normal { from, to } => {
                if let Some(piece) = self.pieces[from.0] {
                    self.pieces[from.0] = None;
                    self.pieces[to.0] = Some(piece);

                    let piece_idx = piece.to_index();
                    self.piece_boards[piece_idx].clear_bit(from.0);
                    self.piece_boards[piece_idx].set_bit(to.0);
                }
            },
            Move::Capture { from, to, captured } => {
                if let Some(piece) = self.pieces[from.0] {
                    let capture_idx = captured.to_index();
                    self.piece_boards[capture_idx].clear_bit(to.0);
                    
                    self.pieces[from.0] = None;
                    self.pieces[to.0] = Some(piece);
                    let piece_idx = piece.to_index();
                    self.piece_boards[piece_idx].clear_bit(from.0);
                    self.piece_boards[piece_idx].set_bit(to.0);
                }
            },
            Move::Promotion { from, to, piece: promoted } => {
                if let Some(_) = self.pieces[from.0] {
                    self.pieces[from.0] = None;
                    let pawn_idx = if self.side_to_move == Color::White { 0 } else { 6 };
                    self.piece_boards[pawn_idx].clear_bit(from.0);

                    self.pieces[to.0] = Some(promoted);
                    let piece_idx = promoted.to_index();
                    self.piece_boards[piece_idx].set_bit(to.0);
                }
            },
            Move::CapturePromotion { from, to, captured, promotion } => {
                if let Some(_) = self.pieces[from.0] {
                    let capture_idx = captured.to_index();
                    self.piece_boards[capture_idx].clear_bit(to.0);
                    
                    self.pieces[from.0] = None;
                    let pawn_idx = if self.side_to_move == Color::White { 0 } else { 6 };
                    self.piece_boards[pawn_idx].clear_bit(from.0);
                    
                    self.pieces[to.0] = Some(promotion);
                    let piece_idx = promotion.to_index();
                    self.piece_boards[piece_idx].set_bit(to.0);
                }
            }
        }
        
        self.side_to_move = !self.side_to_move;
    }

    // returns combined bitboard of one color
    pub fn side_pieces(&self, side: Side) -> BitBoard {
        let mut combined = BitBoard(0);
        for i in side.piece_range() {
            combined.0 |= self.piece_boards[i].0;
        }
        combined
    }

    // returns reference to piece bitboard given side and piece type
    pub fn piece_bb(&self, side: Side, piece: Piece) -> &BitBoard {
        &self.piece_boards[side.piece_range().start + piece as usize]
    }

    pub fn reset(&mut self) {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed = super::parse_fen(fen);
        *self = parsed.into();
    }

    pub fn get_pieces(&self) -> &[Option<Piece>; 64] {
        &self.pieces
    }

    pub fn get_bitboards(&self) -> &[BitBoard; 12] {
        &self.piece_boards
    }

    pub fn occupancy(&self) -> u32 {
        let mut occupied = BitBoard(0);
        for bb in self.piece_boards.iter() {
            occupied.0 |= bb.0;
        }
        occupied.pop_count()
    }

    pub fn get_side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn king_square(&self, side: Color) -> Square {
        let king_bb = match side {
            Color::White => self.piece_boards[5], // White king
            Color::Black => self.piece_boards[11], // Black king
        };
        Square(king_bb.0.trailing_zeros() as usize)
    }

    pub fn has_bishop_pair(&self, side: Color) -> bool {
        let bishop_idx = match side {
            Color::White => 2,
            Color::Black => 8,
        };

        let bishop_bb = self.piece_boards[bishop_idx];
        let bishop_count = bishop_bb.pop_count();

        if bishop_count < 2 {
            return false;
        }
        // check possible pawn promotion to same color, losing pair
        let light_squares = BitBoard(0x55AA55AA55AA55AA);
        let dark_squares = BitBoard(0xAA55AA55AA55AA55);

        let light_bishops = BitBoard(bishop_bb.0 & light_squares.0).pop_count();
        let dark_bishops = BitBoard(bishop_bb.0 & dark_squares.0).pop_count();

        light_bishops > 0 && dark_bishops > 0
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct CastlingRights(pub u8);
impl CastlingRights {
    pub fn empty(&mut self) -> Self {
        self.0 = Castling::NO_CASTLING;
        *self
    }
    pub fn all(&mut self) -> Self {
        self.0 = Castling::ANY_CASTLING;
        *self
    }
}



// {0000}{Black Queen Castle}{Black King}{White Queen}{White King}
pub struct Castling;
impl Castling {
    pub const NO_CASTLING: u8 = 0;
    pub const WHITE_00: u8 = 0b00000001;
    pub const WHITE_000: u8 = 0b00000010;
    pub const BLACK_00: u8 = 0b00000100;
    pub const BLACK_000: u8 = 0b00001000;

    pub const KING_SIDE: u8 = Self::BLACK_00 | Self::WHITE_00;
    pub const QUEEN_SIDE: u8 = Self::BLACK_000 | Self::WHITE_000;
    pub const WHITE_CASTLING: u8 = Self::WHITE_00 | Self::WHITE_000;
    pub const BLACK_CASTLING: u8 = Self::BLACK_00 | Self::BLACK_000;
    pub const ANY_CASTLING: u8 = Self::WHITE_CASTLING | Self::BLACK_CASTLING;
}

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
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed = parse_fen(start_fen);
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
        let fen = "rnbqkbnr/pppppppp/8/PPPPPPPP/pppppppp/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        let parsed = parse_fen(fen);
        let mut board: Board = parsed.into();

        board.reset();
        assert_eq!(board.get_side_to_move(), Color::White);
        assert_eq!(board.occupancy(), 32);

        assert_eq!(board.king_square(Color::White).0, named::E1.0);
        assert_eq!(board.king_square(Color::Black).0, named::E8.0);

        assert!(board.has_bishop_pair(Color::White));
        assert!(board.has_bishop_pair(Color::Black));

        let no_bishop_pair = "rn1qk1nr/pppppppp/8/8/8/8/PPPPPPPP/RN1QK1NR w KQkq - 0 1";
        let parsed = parse_fen(no_bishop_pair);
        let board: Board = parsed.into();
        assert!(!board.has_bishop_pair(Color::White));
        assert!(!board.has_bishop_pair(Color::Black));
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

    // testing move making on board; don't need string to move after testing
    impl Board {
        fn make_move_str(&mut self, move_str: &str) -> Result<(), &'static str> {
            if move_str.len() != 4 {
                return Err("Invalid move format");
            }

            let from_file = (move_str.chars().nth(0).unwrap() as u8 - b'a') as usize;
            let from_rank = (move_str.chars().nth(1).unwrap() as u8 - b'1') as usize;
            let to_file = (move_str.chars().nth(2).unwrap() as u8 - b'a') as usize;
            let to_rank = (move_str.chars().nth(3).unwrap() as u8 - b'1') as usize;

            let from = Square(from_file + from_rank * 8);
            let to = Square(to_file + to_rank * 8);

            self.make_move(Move::Normal { from, to });
            Ok(())
        }
    }

    #[test]
    fn test_make_move() {
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let parsed = parse_fen(start_fen);
        let mut board: Board = parsed.into();

        let print_board = bitboards_to_board(board.piece_boards);
        for rank in &print_board {
            println!("{}", rank.iter().collect::<String>());
        }
        println!("\nAfter moving:\n");

        board.make_move_str("e2e4").unwrap();
        assert!(matches!(board.pieces[named::E2.0], None));
        assert!(matches!(board.pieces[named::E4.0], Some(Piece::WhitePawn)));
        assert_eq!(board.side_to_move, Color::Black);

        let print_board = bitboards_to_board(board.piece_boards);
        for rank in &print_board {
            println!("{}", rank.iter().collect::<String>());
        }
    }
}