use crate::pieces::{process_move_system, send_move_system};
use crate::{is_multiplayer, not_multiplayer};

use super::*;

pub struct SquarePlugin;
impl Plugin for SquarePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<SquareMaterials>()
            .add_message::<ResetSelectedEvent>()
            .add_systems(Startup, create_board)
            .add_systems(
                Update,
                process_move_system
                    .run_if(resource_changed::<SelectedSquare>)
                    .run_if(not_multiplayer),
            )
            .add_systems(
                Update,
                send_move_system
                    .run_if(resource_changed::<SelectedSquare>)
                    .run_if(is_multiplayer),
            )
            .add_systems(
                Update,
                select_piece.run_if(resource_changed::<SelectedSquare>),
            )
            .add_systems(Update, despawn_taken_pieces)
            .add_systems(Update, reset_selected);
    }
}
