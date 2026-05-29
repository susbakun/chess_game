use super::*;

pub fn init_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            LoadingScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font,
                    font_size: 60.0,
                    ..Default::default()
                },
                TextColor(Color::linear_rgb(0.8, 0.8, 0.8)),
            ));
        });
}

/// Update text with the correct turn
pub fn show_loading_screen(
    game_state: Res<GameState>,
    mut background_query: Query<(&mut BackgroundColor, &LoadingScreen, &mut Visibility)>,
) {
    if !game_state.is_loading {
        return;
    }

    println!("Hello is loading");

    for (mut bg_color, _tag, mut visiblity) in background_query.iter_mut() {
        bg_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.7);

        *visiblity = Visibility::Visible;
    }
}

pub fn hide_loading_screen(
    game_state: Res<GameState>,
    mut background_query: Query<(&mut Visibility, &LoadingScreen)>,
) {
    if game_state.is_loading {
        return;
    }

    for (mut visibility, _) in background_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
    return;
}
