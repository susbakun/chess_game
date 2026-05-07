use super::*;

pub fn select_piece(
    selected_square: Res<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
    piece_query: Query<(Entity, &Piece)>,
) {
    let square_entity = if let Some(entity) = 
        selected_square.entity {
            entity
    } else {
        return
    };

    let square = if let Ok(square) = 
        squares_query.get(square_entity) {
            square
    } else {
        return
    };

    if selected_piece.entity.is_none() {
        for (piece_entity, piece) in piece_query.iter() {
            if piece.x == square.x && piece.y == square.y {
                selected_piece.entity = Some(piece_entity);
                break;
            }
        }
    }
    
}


pub fn reset_selected(
    mut message_reader: MessageReader<ResetSelectedEvent>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>
) {
    for _message in message_reader.read() {
        selected_piece.entity = None;
        selected_square.entity = None;
    }
}
