#[cfg(not(target_arch = "wasm32"))]
use crate::engine::StockfishEngine;

use super::*;

pub fn create_start_menu_buttons(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    #[cfg(not(target_arch = "wasm32"))]
    parent.spawn((
        PlayWithAiButton,
        Button,
        Node {
            width: Val::Px(300.0),
            height: Val::Px(80.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::linear_rgb(0.2, 0.5, 0.8)),
        children![
            Text::new("Play with AI"),
            TextFont {
                font: font.clone(),
                font_size: 40.0,
                ..Default::default()
            }
        ],
    ));

    parent.spawn((
        PlayOfflineButton,
        Button,
        Node {
            width: Val::Px(300.0),
            height: Val::Px(80.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::linear_rgb(0.2, 0.5, 0.8)),
        children![
            Text::new("Play offline"),
            TextFont {
                font: font.clone(),
                font_size: 40.0,
                ..Default::default()
            }
        ],
    ));

    parent.spawn((
        ExitButton,
        Button,
        Node {
            width: Val::Px(300.0),
            height: Val::Px(80.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::linear_rgb(0.2, 0.5, 0.8)),
        children![
            Text::new("Exit"),
            TextFont {
                font: font.clone(),
                font_size: 40.0,
                ..Default::default()
            }
        ],
    ));
}

pub fn start_menu_buttons_interactions_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut Button),
        Changed<Interaction>,
    >,
) {
    for (entity, interaction, mut bg_color, mut button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                input_focus.set(entity);
                *bg_color = BackgroundColor(HOVERED_START_MENU_BUTTON);

                // The accessibility system's only update
                // the button's state when the `Button`
                // component is marked as changed.
                button.set_changed();
            }
            Interaction::Pressed => {
                input_focus.set(entity);
                *bg_color = BackgroundColor(PRESSED_START_MENU_BUTTON);

                // The accessibility system's only update
                // the button's state when the `Button`
                // component is marked as changed.
                button.set_changed();
            }
            Interaction::None => {
                input_focus.set(entity);
                *bg_color = BackgroundColor(NORMAL_START_MENU_BUTTON);

                // The accessibility system's only update
                // the button's state when the `Button`
                // component is marked as changed.
                button.set_changed();
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn play_with_ai_button_system(
    mut game_state: ResMut<GameState>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayWithAiButton>)>,
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.game_type = Some(GameType::PlayWithAi);

            let engine = StockfishEngine::new().expect("failed to start the engine");
            game_state.engine = Some(engine);
        }
    }
}

pub fn play_offline_button_system(
    mut game_state: ResMut<GameState>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayOfflineButton>)>,
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.game_type = Some(GameType::PlayOffline)
        }
    }
}

pub fn exit_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            std::process::exit(0);
        }
    }
}
