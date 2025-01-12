use super::*;

pub fn fen_to_bb(fen: String) -> [BitBoard; 12] {
    let mut piece_bitboards = [BitBoard(0); 12];

    let parts: Vec<&str> = fen.split_whitespace().collect();
    let position = parts[0];

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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn correct_start_position() {
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let piece_bitboards = fen_to_bb(start_fen.to_string());
        let board = bitboards_to_board(piece_bitboards);
        
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

        // Print board for visual verification
        for rank in &board {
            println!("{}", rank.iter().collect::<String>());
        }
    }
}