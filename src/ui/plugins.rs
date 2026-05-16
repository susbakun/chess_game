
use super::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_next_move_text)
            .add_systems(Startup, init_end_menu)
            .add_systems(Startup, init_start_menu)
            .add_systems(Startup, init_difficulty_menu)
            .add_systems(Update, next_move_text_update
                .run_if(resource_changed::<GameState>))
            .add_systems(Update, (
                show_end_menu,
                hide_end_menu
            )
                .run_if(resource_changed::<GameState>))
            .add_systems(Update, (
                start_menu_buttons_interactions_system,
                back_to_menu_button_system
            ))
            .add_systems(Update, (
                show_start_menu,
                hide_start_menu
            ))
            .add_systems(Update, (
                end_menu_buttons_interactions_system,
                #[cfg(not(target_arch = "wasm32"))]
                play_with_ai_button_system,
                play_offline_button_system,
                exit_button_system
            ))
            .add_systems(Update, (
                #[cfg(not(target_arch = "wasm32"))]
                difficulty_menu_buttons_interactions_system,
                #[cfg(not(target_arch = "wasm32"))]
                hard_difficulty_button_system,
                #[cfg(not(target_arch = "wasm32"))]
                medium_difficulty_button_system,
                #[cfg(not(target_arch = "wasm32"))]
                easy_difficulty_button_system
            ))
            .add_systems(Update, (
                #[cfg(not(target_arch = "wasm32"))]
                show_difficulty_menu,
                #[cfg(not(target_arch = "wasm32"))]
                hide_difficulty_menu
            ));
    }
}