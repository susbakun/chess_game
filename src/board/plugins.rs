use super::*;

pub struct SquarePlugin;
impl Plugin for SquarePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerTurn>()
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<SquareMaterials>()
            .init_resource::<GoneCount>()
            .add_message::<ResetSelectedEvent>()
            .add_systems(Startup, create_board)
            .add_systems(Update, move_piece
                .run_if(resource_changed::<SelectedSquare>))
            .add_systems(Update, select_piece
                .run_if(resource_changed::<SelectedSquare>))
            .add_systems(Update, despawn_taken_pieces)
            .add_systems(Update, reset_selected);

    }
}