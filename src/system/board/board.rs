// unified struct of all piece bitboards and state
use super::{ BitBoard, Color, ParsedFen, Piece, Side, Square };

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

