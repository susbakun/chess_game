use super::*;

pub fn color_of_square(pos: (i8, i8), pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color);
        }
    }

    None
}

pub fn is_path_empty(begin: (i8, i8), end: (i8, i8), pieces: &Vec<Piece>) -> bool {
    // same col
    if begin.0 == end.0 {
        for piece in pieces {
            if piece.x == begin.0
                && ((piece.y > begin.1 && piece.y < end.1)
                    || (piece.y > end.1 && piece.y < begin.1))
            {
                return false;
            }
        }
    }
    // same row
    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    || (piece.x > end.0 && piece.x < begin.0))
            {
                return false;
            }
        }
    }

    // Diagnols
    let x_diff = (begin.0 - end.0).abs();
    let y_diff = (begin.1 - end.1).abs();

    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.0 < end.0 && begin.1 < end.1 {
                (begin.0 + i, begin.1 + i)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                (begin.0 - i, begin.1 + i)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                (begin.0 + i, begin.1 - i)
            } else {
                (begin.0 - i, begin.1 - i)
            };

            if color_of_square(pos, pieces).is_some() {
                return false;
            }
        }
    }

    true
}

pub fn is_king_side_castling(pos: (i8, i8)) -> bool {
    // checking two positions that a king would
    // go to for king-side castling

    [
        WHITE_KING_POS_QUEEN_SIDE_CASTLING,
        BLACK_KING_POS_KING_SIDE_CASTLING,
    ]
    .contains(&pos)
}

pub fn convert_to_fen(pieces: &Vec<Piece>, current_color: PieceColor) -> String {
    let mut fen = String::new();

    for rank in (0..8).rev() {
        let mut empty_count = 0;
        for file in 0..8 {
            let piece_at_square = pieces
                .iter()
                .filter(|p| !p.taken)
                .find(|p| p.x == rank && p.y == file);

            match piece_at_square {
                Some(piece) => {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    let fen_piece = piece_to_fen_character(piece);
                    fen.push(fen_piece);
                }
                None => {
                    empty_count += 1;
                }
            }
        }

        if empty_count > 0 {
            fen.push_str(&empty_count.to_string());
        }

        if rank > 0 {
            fen.push('/');
        }
    }

    let turn = match current_color {
        PieceColor::White => "w",
        PieceColor::Black => "b",
    };

    fen.push_str(&format!(" {turn} KQkq - 0 1"));

    fen
}

fn piece_to_fen_character(piece: &Piece) -> char {
    match (piece.piece_type, piece.color) {
        // White pieces (uppercase)
        (PieceType::Knight, PieceColor::White) => 'N',
        (PieceType::Rook, PieceColor::White) => 'R',
        (PieceType::Bishop, PieceColor::White) => 'B',
        (PieceType::Queen, PieceColor::White) => 'Q',
        (PieceType::King, PieceColor::White) => 'K',
        (PieceType::Pawn, PieceColor::White) => 'P',

        // Black pieces (lowercase)
        (PieceType::Knight, PieceColor::Black) => 'n',
        (PieceType::Rook, PieceColor::Black) => 'r',
        (PieceType::Bishop, PieceColor::Black) => 'b',
        (PieceType::Queen, PieceColor::Black) => 'q',
        (PieceType::King, PieceColor::Black) => 'k',
        (PieceType::Pawn, PieceColor::Black) => 'p',
    }
}

pub fn fen_to_piece_pos(fen: String) -> Option<((i8, i8), (i8, i8))> {
    let mut chars = fen.chars();

    let start_file = chars.next().map(map_file_to_square_num)?;
    // we have to decrease it by one because the rank starts from 0
    // in our grame
    let start_rank = (chars.next()?.to_digit(10)? as i8) - 1;
    let start_pos = (start_rank, start_file);

    let end_file = chars.next().map(map_file_to_square_num)?;
    // we have to decrease it by one because the rank starts from 0
    // in our grame
    let end_rank = (chars.next()?.to_digit(10)? as i8) - 1;
    let end_pos = (end_rank, end_file);

    Some((start_pos, end_pos))
}

fn map_file_to_square_num(file: char) -> i8 {
    match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => 0,
    }
}

// format it this way "mm:ss"
pub fn format_time(seconds: u16) -> String {
    let minutes = seconds / 60;
    let secs = seconds % 60;

    format!("{:02}:{:02}", minutes, secs)
}
