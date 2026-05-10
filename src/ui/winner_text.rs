use super::*;


/// show text with the correct winner
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
            Text::new(""),
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
    });
}


/// Update text with the correct turn
pub fn show_winner_text(
    game_over: Res<GameOver>,
    winner: Res<Winner>,
    mut text_query: Query<(&mut Text, &WinnerText)>,
    mut background_query: Query<(&mut BackgroundColor, &GameOverScreen, &mut Visibility)>
) {
    if !game_over.0 {
        return
    }

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

    let winner_text = match winner.0 {
        PieceColor::White => "White won",
        PieceColor::Black => "Black won"
    };



    for (mut text, _tag) in 
        text_query.iter_mut() {
            text.0 = winner_text.to_string();
    }
}
