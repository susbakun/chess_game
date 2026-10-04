use super::*;

#[cfg(not(target_arch = "wasm32"))]
/// initilizing the end screen
pub fn init_waiting_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            Node {
                position_type: PositionType::Relative,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            Visibility::Hidden,
            WaitingScrren,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Waiting..."),
                TextFont {
                    font,
                    font_size: 60.0,
                    ..Default::default()
                },
                TextColor(Color::linear_rgb(0.8, 0.8, 0.8)),
                WaitingText,
            ));
        });
}

#[cfg(not(target_arch = "wasm32"))]
/// Update text with the correct turn
pub fn show_waiting_screen(
    game_state: Res<ClientGameState>,
    mut background_query: Query<(&mut BackgroundColor, &WaitingScrren, &mut Visibility)>,
) {
    if game_state.game_type != Some(GameType::Multiplayer) {
        return;
    }

    if !game_state.waiting_for_opponent {
        return;
    }

    for (mut bg_color, _tag, mut visiblity) in background_query.iter_mut() {
        bg_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.7);

        *visiblity = Visibility::Visible;
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn hide_waiting_screen(
    game_state: Res<ClientGameState>,
    mut background_query: Query<(&mut Visibility, &WaitingScrren)>,
) {
    if game_state.waiting_for_opponent {
        return;
    }

    for (mut visibility, _) in background_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
    return;
}
