use crate::network::MyColor;

use super::*;

/// Initialize the text
pub fn init_next_move_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Next move: White"),
                TextFont {
                    font,
                    font_size: 40.0,
                    ..Default::default()
                },
                TextColor(Color::linear_rgb(0.8, 0.8, 0.8)),
                NextMoveText,
            ));
        });
}

pub fn init_timers_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let initial = format_time(TIMER_DURATION_SECS);

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            bottom: Val::Px(10.0),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new(initial.clone()),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(Color::linear_rgb(0.8, 0.8, 0.8)),
                Visibility::Hidden,
                TimerPosition::Bottom,
            ));
        });

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            top: Val::Px(10.0),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new(initial),
                TextFont {
                    font,
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(Color::linear_rgb(0.8, 0.8, 0.8)),
                Visibility::Hidden,
                TimerPosition::Top,
            ));
        });
}

pub fn update_timers_text(
    game_state: Res<ClientGameState>,
    my_color: Option<Res<MyColor>>,
    mut players_query: Query<(&mut Text, &mut Visibility, &TimerPosition)>,
) {
    let my_color = my_color.map(|c| c.0).unwrap_or(PieceColor::White);

    let ai_mode = game_state.game_type == Some(GameType::PlayWithAi);

    for (mut text, mut visibility, position) in players_query.iter_mut() {
        let color_shown = match (position, my_color) {
            (TimerPosition::Bottom, c) => c,
            (TimerPosition::Top, PieceColor::White) => PieceColor::Black,
            (TimerPosition::Top, PieceColor::Black) => PieceColor::White,
        };

        text.0 = format_time(match color_shown {
            PieceColor::White => game_state.timer.0,
            PieceColor::Black => game_state.timer.1,
        });

        *visibility = if ai_mode {
            Visibility::Hidden
        } else {
            Visibility::Visible
        }
    }
}

/// Update text with the correct turn
pub fn next_move_text_update(
    game_state: Res<ClientGameState>,
    mut query: Query<(&mut Text, &NextMoveText)>,
) {
    for (mut text, _tag) in query.iter_mut() {
        text.0 = format!(
            "Next move: {}",
            match game_state.player.0 {
                PieceColor::White => "White",
                PieceColor::Black => "Black",
            }
        )
    }
}
