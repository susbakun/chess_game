use super::*;

pub fn render_taken_on_side(
    commands: Commands,
    mut piece: Piece,
    material: Handle<StandardMaterial>,
    gone_count: (u8, u8),
    piece_handles: PieceHandles,
) {
    if piece.color == PieceColor::White {
        piece.x = 8 - (gone_count.0 as i8 % 8);
        piece.y = 9 + (gone_count.0 as i8 / 8);
    } else {
        piece.x = 8 - (gone_count.1 as i8 % 8);
        piece.y = -2 - (gone_count.1 as i8 / 8);
    }

    piece.taken = true;

    
    spawn_piece(
        commands,
        material, 
        piece, 
        piece_handles
    );
}