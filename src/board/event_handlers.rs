use super::*;

pub fn on_square_hover(
    _over: On<Pointer<Over>>,
    selected_square: Res<SelectedSquare>,
    materials: Res<SquareMaterials>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Square)>,
) {
    // Don't reset color if this square is selected
    if let Some(selected_square_entity) = 
        selected_square.entity {
        if selected_square_entity == _over.entity {
            return;
        }
    }

    if let Ok((mut material_handle, _square)) = 
        query.get_mut(_over.entity) {
            material_handle.0 = materials.highlight_color.clone();
    }
}

pub fn on_square_hover_end(
    _out: On<Pointer<Out>>,
    materials: ResMut<SquareMaterials>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Square)>,
    selected_square: Res<SelectedSquare>
) {
    // Don't reset color if this square is selected
    if let Some(selected_square_entity) = 
        selected_square.entity {
        if selected_square_entity == _out.entity {
            return;
        }
    }

    if let Ok((mut material_handle, square)) = 
        query.get_mut(_out.entity) {
        material_handle.0 = if square.is_white() {
            materials.white_color.clone()
        } else {
            materials.black_color.clone()
        };
    }
}

pub fn on_sqaure_click(
    _click: On<Pointer<Click>>,
    materials: Res<SquareMaterials>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut squares_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Square)>
) {
    if _click.button != PointerButton::Primary {
        return
    }   

    // reset the previous selected square
    if let Some(prev_square_entity) = 
        selected_square.entity {
        if prev_square_entity != _click.entity {
            if let Ok((mut prev_material, prev_square)) = 
                squares_query.get_mut(prev_square_entity) {
                prev_material.0 = if prev_square.is_white() {
                    materials.white_color.clone()
                } else {
                    materials.black_color.clone()
                };
            }
        }
    }

    if let Ok((mut material_handle, _square)) = 
        squares_query.get_mut(_click.entity) {
        // Highlight clicked square
        material_handle.0 = materials.selected_color.clone();
        selected_square.entity = Some(_click.entity);

    } else {
        // Player clicked outside the board, deselect everything
        selected_piece.entity = None;
        selected_square.entity = None;
    }
}

pub fn despawn_taken_pieces(
    mut commands: Commands,
    materials: Res<SquareMaterials>,
    piece_handles: Res<PieceHandles>,
    mut game_state: ResMut<GameState>,
    mut query: Query<(Entity, &mut Piece, &Taken)>
) {

    for (
        entity, 
        piece, 
        _taken
    ) in 
        query.iter_mut() 
        {
            commands.entity(entity).despawn();

            let material = if piece.color == PieceColor::White {
                game_state.removed_counts.0 += 1;
                materials.white_color.clone()
            } else {
                game_state.removed_counts.1 += 1;
                materials.black_color.clone()
            };
            
            render_taken_on_side(
                commands.reborrow(),
                *piece,
                material,
                game_state.removed_counts,
                piece_handles.clone()
            );
    }
}