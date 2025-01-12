use super::{prng::LcgRng, Piece, Color};

const NUM_PIECES: usize = 12;
const NUM_SQUARES: usize = 64;
const NUM_CASTLING_RIGHTS: usize = 4;
const NUM_EN_PASSANT_FILES: usize = 8;

pub struct Zobrist {
    piece_square: [[u64; NUM_SQUARES]; NUM_PIECES],
    castling_rights: [u64; NUM_CASTLING_RIGHTS],
    en_passant_file: [u64; NUM_EN_PASSANT_FILES],
    side_to_move: u64,
}

pub struct ZobBoard {
    pub pieces: [Option<Piece>; 64],
    pub castling_rights: u8,
    pub en_passant_file: Option<u8>,
    pub side_to_move: Color,
}

impl Zobrist {
    pub fn new(seed: u64) -> Self {
        let mut rng = LcgRng::new(seed);
        let mut piece_square = [[0u64; NUM_SQUARES]; NUM_PIECES];
        let mut castling_rights = [0u64; NUM_CASTLING_RIGHTS];
        let mut en_passant_file = [0u64; NUM_EN_PASSANT_FILES];

        for piece in 0..NUM_PIECES {
            for square in 0..NUM_SQUARES {
                piece_square[piece][square] = rng.next_u64();
            }
        }

        for i in 0..NUM_CASTLING_RIGHTS {
            castling_rights[i] = rng.next_u64();
        }

        for i in 0..NUM_EN_PASSANT_FILES {
            en_passant_file[i] = rng.next_u64();
        }

        let side_to_move = rng.next_u64();

        Zobrist {
            piece_square,
            castling_rights,
            en_passant_file,
            side_to_move,
        }
    }

    pub fn hash(&self, board: &ZobBoard) -> u64 {
        let mut hash = 0u64;

        for (square, piece) in board.pieces.iter().enumerate() {
            if let Some(piece) = piece {
                let piece_index = piece.to_index();
                hash ^= self.piece_square[piece_index][square];
            }
        }

        hash ^= self.castling_rights[board.castling_rights as usize];

        if let Some(file) = board.en_passant_file {
            hash ^= self.en_passant_file[file as usize];
        }

        if board.side_to_move == Color::Black {
            hash ^= self.side_to_move;
        }

        hash
    }
}