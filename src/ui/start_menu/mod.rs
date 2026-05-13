use super::*;

mod buttons;
pub use buttons::*;

pub fn init_start_menu(
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
    Visibility::Visible,
    MenuScreen
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Chess game"),
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

        create_start_menu_buttons(parent, asset_server);
    });
}

pub fn show_start_menu(
    game_state: Res<GameState>,
    mut background_query: Query<(&mut Visibility, &MenuScreen)>
) {
    if game_state.game_type.is_none() {
        for (mut visibility, _) in 
            background_query.iter_mut() {
                *visibility = Visibility::Visible;
        }
    }
}


pub fn hide_start_menu(
    game_state: Res<GameState>,
    mut background_query: Query<(&mut Visibility, &MenuScreen)>
) {
    if let Some(_) = &game_state.game_type {
        for (mut visibility, _) in 
            background_query.iter_mut() {
                *visibility = Visibility::Hidden;
        }
    }
}