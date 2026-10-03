use bevy::color::palettes::css::{BLACK, BLUE, WHITE};
use bevy::input_focus::InputFocus;

use super::*;

pub fn create_end_menu_buttons(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    parent.spawn((
        ReplayButton,
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
        BackgroundColor(NORMAL_END_MENU_BUTTON),
        BorderColor::all(Color::WHITE),
        children![
            Text::new("Back to menu"),
            TextFont {
                font,
                font_size: 33.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            TextShadow::default(),
        ],
    ));
}

pub fn end_menu_buttons_interactions_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
        ),
        (Changed<Interaction>, With<ReplayButton>),
    >,
) {
    for (entity, interaction, mut bg_color, mut border_color, mut button) in
        interaction_query.iter_mut()
    {
        match *interaction {
            Interaction::Hovered => {
                input_focus.set(entity);
                *bg_color = HOVERED_END_MENU_BUTTON.into();
                *border_color = BorderColor::all(WHITE);

                // The accessibility system's only update
                // the button's state when the `Button`
                // component is marked as changed.
                button.set_changed();
            }
            Interaction::Pressed => {
                input_focus.set(entity);
                *bg_color = PRESSED_END_MENU_BUTTON.into();
                *border_color = BorderColor::all(BLUE);

                // The accessibility system's only update
                // the button's state when the `Button`
                // component is marked as changed.
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                *bg_color = NORMAL_END_MENU_BUTTON.into();
                *border_color = BorderColor::all(BLACK);

                // The accessibility system's only update
                // the button's state when the `Button`
                // component is marked as changed.
                button.set_changed();
            }
        }
    }
}

pub fn back_to_menu_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ReplayButton>)>,
    mut replay_event_writer: MessageWriter<BackToMenuEvent>,
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            replay_event_writer.write(BackToMenuEvent);
        }
    }
}
