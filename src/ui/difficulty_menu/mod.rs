use crate::ui::difficulty_menu::buttons::create_difficulty_menu_buttons;

use super::*;

mod buttons;
pub use buttons::*;

pub fn init_difficulty_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
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
    BackgroundColor(Color::linear_rgb(
        0.1, 
        0.1, 
        0.15
    )),
    Visibility::Hidden,
    DifficultyMenuScreen
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("AI Difficulty"),
            TextFont {
                font: font.clone(),
                font_size: 80.0,
                ..Default::default()
            },
            TextColor(Color::linear_rgb(
                0.8, 
                0.8, 
                0.8
            )),
            Node {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..Default::default()
            }
        ));

        create_difficulty_menu_buttons(parent, asset_server);
    });
}

pub fn show_difficulty_menu(
    game_state: Res<GameState>,
    mut background_query: Query<(&mut Visibility, &DifficultyMenuScreen)>
) {
    if let Some(game_type) = &game_state.game_type {
        if *game_type == GameType::PlayWithAi && 
            game_state.difficulty.is_none() {
                for (mut visibility, _) in 
                    background_query.iter_mut() {
                        *visibility = Visibility::Visible;
                }
        }
    }
}


pub fn hide_difficulty_menu(
    game_state: Res<GameState>,
    mut background_query: Query<(&mut Visibility, &DifficultyMenuScreen)>
) {
    if let Some(game_type) = &game_state.game_type {
        if *game_type == GameType::PlayWithAi && 
            game_state.difficulty.is_some() {
                for (mut visibility, _) in 
                    background_query.iter_mut() {
                        *visibility = Visibility::Hidden;
                }
        }
    }
}