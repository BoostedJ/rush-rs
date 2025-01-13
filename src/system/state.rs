use super::parse_fen;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub half_move: u8,
    pub stm: usize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CastlingRights(pub u8);
impl CastlingRights {
    pub fn empty() -> Self {
        Self(Castling::NO_CASTLING)
    }
    pub fn all() -> Self {
        Self::default()
    }
}

impl State {
    pub fn from_fen(fen: &str) -> Self {
        let parsed = parse_fen(fen);

        Self {
            castling_rights: parsed.castling_rights,
            en_passant: parsed.en_passant,
            half_move: 0,
            stm: parsed.side_to_move as usize,
        }
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self(Castling::ANY_CASTLING)
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

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Square(pub usize);

#[repr(usize)]
#[rustfmt::skip]
pub enum SquareLabels {
    None,
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}
/* 
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    bb_sides: [BitBoard; 2],
    bb_pieces: [[BitBoard; 6]; 2],
    state: State,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "White pieces:")?;
        for piece in &self.bb_pieces[Sides::WHITE] {
            writeln!(f, "{}", piece)?;
        }
        writeln!(f, "Black pieces:")?;
        for piece in &self.bb_pieces[Sides::BLACK] {
            writeln!(f, "{}", piece)?;
        }
        writeln!(f, "State: {:?}", self.state)
    }
}
*/