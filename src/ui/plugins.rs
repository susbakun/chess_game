
use super::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_next_move_text)
            .add_systems(Startup, init_end_menu)
            .add_systems(Startup, init_start_menu)
            .add_systems(Update, next_move_text_update
                .run_if(resource_changed::<GameState>))
            .add_systems(Update, (
                show_end_menu,
                hide_end_menu
            )
                .run_if(resource_changed::<GameState>))
            .add_systems(Update, (
                start_menu_buttons_interactions_system,
                replay_button_system
            ))
            .add_systems(Update, (
                show_start_menu,
                hide_start_menu
            ))
            .add_systems(Update, (
                end_menu_buttons_interactions_system,
                play_with_ai_button_system,
                play_offline_button_system,
                exit_button_system
            ));
    }
}