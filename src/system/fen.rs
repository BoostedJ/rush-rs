use super::{ BitBoard, Castling, CastlingRights, Color, Square };

pub struct ParsedFen {
    pub piece_boards: [BitBoard; 12],
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub half_move: u8,
    pub full_move: u16,
}

pub fn parse_fen(fen: &str) -> ParsedFen {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() != 6 {
        panic!("Invalid FEN string");
    }

    let piece_boards = fen_to_bb(parts[0].to_string());

    let side_to_move = match parts[1] {
        "w" => Color::White,
        "b" => Color::Black,
        _ => panic!("Invalid side to move. Use FEN ('w'/'b')"),
    };

    let mut castling = 0u8;
    for c in parts[2].chars() {
        match c {
            'K' => castling |= Castling::WHITE_00,
            'Q' => castling |= Castling::WHITE_000,
            'k' => castling |= Castling::BLACK_00,
            'q' => castling |= Castling::BLACK_000,
            '-' => break,
            _ => panic!("Invalid castling rights. Use FEN (KQkq)"),
        }
    }

    let castling_rights = CastlingRights(castling);

    let en_passant = match parts[3] {
        "-" => None,
        square => Some(parse_square(square)),
    };

    let half_move = parts[4].parse::<u8>()
        .expect("Invalid halfmove clock");

    let full_move = parts[5].parse::<u16>()
        .expect("Invalid fullmove number");

    ParsedFen {
        piece_boards,
        side_to_move,
        castling_rights,
        en_passant,
        half_move,
        full_move,
    }
}

fn fen_to_bb(position: String) -> [BitBoard; 12] {
    let mut piece_bitboards = [BitBoard(0); 12];

    let mut rank = 7;
    let mut file = 0;

    for c in position.chars() {
        match c {
            '/' => {
                rank -= 1;
                file = 0;
            }
            '1'..'8' => {
                file += c.to_digit(10).unwrap() as usize;
            }
            piece => {
                if let Some(piece_index) = char_to_piece(piece) {
                    let square = rank * 8 + file;
                    piece_bitboards[piece_index].0 |= 1u64 << square;
                }
                file += 1;
            }
        }
    }
    piece_bitboards
}

fn parse_square(s: &str) -> Square {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() != 2 {
        panic!("Invalid square notation");
    }

    let file = (chars[0] as u8 - b'a') as usize;
    let rank = (chars[1] as u8 - b'1') as usize;

    Square(rank * 8 + file)
}

fn char_to_piece(c: char) -> Option<usize> {
    match c {
        'P' => Some(0),  // White Pawn
        'N' => Some(1),  // White Knight
        'B' => Some(2),  // White Bishop
        'R' => Some(3),  // White Rook
        'Q' => Some(4),  // White Queen
        'K' => Some(5),  // White King
        'p' => Some(6),  // Black Pawn
        'n' => Some(7),  // Black Knight
        'b' => Some(8),  // Black Bishop
        'r' => Some(9),  // Black Rook
        'q' => Some(10), // Black Queen
        'k' => Some(11), // Black King
        _ => None
    }
}

