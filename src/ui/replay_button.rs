use bevy::input_focus::InputFocus;
use bevy::color::palettes::css::{BLACK, BLUE, WHITE};

use super::*;


/// seting up the replay button
pub fn create_replay_button(
    asset_server: Res<AssetServer>,
) -> impl Bundle {
    let font = asset_server.load(
        "fonts/FiraSans-Bold.ttf"
    );

    (
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(60.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..Default::default()
        },
        BackgroundColor(NORMAL_BUTTON),
        BorderColor::all(Color::WHITE),
        children![
            Text::new("Repaly"),
            TextFont {
                font,
                font_size: 33.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            TextShadow::default(),
        ]
    )
}

pub fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<(
        Entity,
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut Button,
    ),
    Changed<Interaction>
    >
) {
    for (entity, interaction, mut bg_color, 
        mut border_color, mut button) in 
        interaction_query.iter_mut() {
            match *interaction {
                Interaction::Hovered => {
                    input_focus.set(entity);
                    *bg_color = HOVERED_BUTTON.into();
                    *border_color = BorderColor::all(WHITE);

                    // The accessibility system's only update 
                    // the button's state when the `Button` 
                    // component is marked as changed.
                    button.set_changed();
                }
                Interaction::Pressed => {
                    input_focus.set(entity);
                    *bg_color = PRESSED_BUTTON.into();
                    *border_color = BorderColor::all(BLUE);

                    // The accessibility system's only update 
                    // the button's state when the `Button` 
                    // component is marked as changed.
                    button.set_changed();
                    restart_game();
                }
                Interaction::None => {
                    input_focus.clear();
                    *bg_color = NORMAL_BUTTON.into();
                    *border_color = BorderColor::all(BLACK);

                    // The accessibility system's only update 
                    // the button's state when the `Button` 
                    // component is marked as changed.
                    button.set_changed();
                }
            }
    }
}


fn restart_game() {

}