use super::*;

#[derive(Message)]
pub struct BackToMenuEvent;

pub fn replay(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    materials: ResMut<Assets<StandardMaterial>>,
    piece_handles: Res<PieceHandles>,
    meshes: ResMut<Assets<Mesh>>,
    square_materials: Res<SquareMaterials>,
    pieces_query: Query<Entity, With<Piece>>,
    squares_query: Query<Entity, With<Square>>,
    taken_pieces_query: Query<Entity, With<Taken>>,
    mut replay_events: MessageReader<BackToMenuEvent>,
) {
    if let Some(_) = replay_events.read().next() {
        *game_state = GameState::default();
    
        for entity in pieces_query.iter() {
            commands.entity(entity).despawn();
        }
        
        for entity in squares_query.iter() {
            commands.entity(entity).despawn();
        }
        
        for entity in taken_pieces_query.iter() {
            commands.entity(entity).despawn();
        }
        
    
        create_board(
            commands.reborrow(), 
            meshes, 
            square_materials
        );
    
        create_pieces(
            commands,
            materials, 
            piece_handles
        );
    }
}