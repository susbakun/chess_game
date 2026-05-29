use super::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_next_move_text)
            .add_systems(Startup, init_end_menu)
            .add_systems(Startup, init_start_menu)
            .add_systems(Startup, init_loading_screen)
            .add_systems(
                Update,
                next_move_text_update.run_if(resource_changed::<GameState>),
            )
            .add_systems(
                Update,
                (
                    start_menu_buttons_interactions_system,
                    play_online_button_system,
                    play_offline_button_system,
                    exit_button_system,
                ),
            )
            .add_systems(Update, (show_start_menu, hide_start_menu))
            .add_systems(
                Update,
                (
                    end_menu_buttons_interactions_system,
                    back_to_menu_button_system,
                ),
            )
            .add_systems(
                Update,
                (show_end_menu, hide_end_menu).run_if(resource_changed::<GameState>),
            )
            .add_systems(
                Update,
                (show_loading_screen, hide_loading_screen).run_if(resource_changed::<GameState>),
            );

        #[cfg(not(target_arch = "wasm32"))]
        {
            app.add_systems(Startup, init_difficulty_menu)
                .add_systems(Update, play_with_ai_button_system)
                .add_systems(
                    Update,
                    (
                        difficulty_menu_buttons_interactions_system,
                        hard_difficulty_button_system,
                        medium_difficulty_button_system,
                        easy_difficulty_button_system,
                    ),
                )
                .add_systems(Update, (show_difficulty_menu, hide_difficulty_menu));
        }
    }
}
