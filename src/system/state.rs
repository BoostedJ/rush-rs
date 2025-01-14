use super::{ parse_fen, Zobrist, CastlingRights, Square, Piece, BitBoard };

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub half_move: u8,
    pub stm: usize,
    pub zobrist_key: u64,
    pub phase: GamePhase,
    pub psqt_score: i32,
    pub last_move: Option<Move>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum GamePhase {
    Opening,
    MiddleGame,
    EndGame,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
    pub capture: Option<Piece>,
}

impl State {
    pub fn from_fen(fen: &str, zobrist: &Zobrist) -> Self {
        let parsed = parse_fen(fen);

        Self {
            castling_rights: parsed.castling_rights,
            en_passant: parsed.en_passant,
            half_move: 0,
            stm: parsed.side_to_move as usize,
            zobrist_key: zobrist.hash(&parsed.into()),
            phase: GamePhase::Opening,
            psqt_score: 0,
            last_move: None,
        }
    }

    pub fn make_move(&mut self, mv: Move, zobrist: &Zobrist) {
        self.last_move = Some(mv);
        //self.zobrist_key ^= zobrist.hash(&self.into());
    }

    pub fn evaluate_phase(&self, material: &[BitBoard; 12]) -> GamePhase {
        todo!()
    }

    pub fn calculate_psqt(&self, piece_boards: &[BitBoard; 12]) -> i32 {
        todo!()
    }
}
