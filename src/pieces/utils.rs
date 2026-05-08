use super::*;


pub fn color_of_square(pos: (i8, i8), pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color)
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
                    ||  (piece.y > end.1 && piece.y  < begin.1))
            {
                    return false
            }
        }
    }
    // same row
    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    ||  (piece.x > end.0 && piece.x  < begin.0))
            {
                    return false
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
                return false
            }
        }
    }

    true
}


pub fn is_king_side_castling(pos: (i8, i8)) -> bool {
    // checking two positions that a king would
    // go to for king-side castling

    pos == (0, 6) || pos == (7, 6)
}