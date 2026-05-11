use super::*;


/// initilizing the end screen
pub fn init_winner_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(
        "fonts/FiraSans-Bold.ttf"
    );

    commands.spawn((
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
    BackgroundColor(Color::srgba(
        0.0, 
        0.0, 
        0.0, 
        0.0
    )),
    Visibility::Hidden,
    GameOverScreen
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("White won"),
            TextFont {
                font,
                font_size: 60.0,
                ..Default::default()
            },
            TextColor(Color::linear_rgb(
                0.8, 
                0.8, 
                0.8)),
            WinnerText,
        ));

        let replay_button_bundle = 
            create_replay_button(asset_server);

        parent.spawn(replay_button_bundle);
    });
}


/// Update text with the correct turn
pub fn show_winner_text(
    game_state: Res<GameState>,
    mut text_query: Query<(&mut Text, &WinnerText)>,
    mut background_query: Query<(&mut BackgroundColor, &GameOverScreen, &mut Visibility)>
) {
    let (game_over, winner) = 
        (game_state.game_over, game_state.winner);

    if !game_over {
        return
    }

    if let Some(winner) = winner {
        for (mut bg_color, _tag, mut visiblity) in 
        background_query.iter_mut() {
            bg_color.0 = Color::srgba(
                0.0, 
                0.0, 
                0.0, 
                0.7
            );

            *visiblity = Visibility::Visible;
        }

        let winner_text = match winner {
            PieceColor::White => "White won",
            PieceColor::Black => "Black won"
        };



        for (mut text, _tag) in 
            text_query.iter_mut() {
                text.0 = winner_text.to_string();
        }   
    }
}
