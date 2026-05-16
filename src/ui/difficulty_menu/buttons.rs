use super::*;


#[cfg(not(target_arch = "wasm32"))]
pub fn create_difficulty_menu_buttons(
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    asset_server: Res<AssetServer>
){
    let font = asset_server.load(
        "fonts/FiraSans-Bold.ttf"
    );
    parent.spawn((
        HardDiff,
        Button,
        Node {
            width: Val::Px(300.0),
            height: Val::Px(80.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::linear_rgb(
            0.2, 
            0.5, 
            0.8
        )),
        children![
            Text::new("Hard"),
            TextFont {
                font: font.clone(),
                font_size: 40.0,
                ..Default::default()
            }
        ]
    ));

    parent.spawn((
        MediumDiff,
        Button,
        Node {
            width: Val::Px(300.0),
            height: Val::Px(80.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::linear_rgb(
            0.2, 
            0.5, 
            0.8
        )),
        children![
            Text::new("Medium"),
            TextFont {
                font: font.clone(),
                font_size: 40.0,
                ..Default::default()
            }
        ]
    ));

    parent.spawn((
        EasyDiff,
        Button,
        Node {
            width: Val::Px(300.0),
            height: Val::Px(80.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::linear_rgb(
            0.2, 
            0.5, 
            0.8
        )),
        children![
            Text::new("Easy"),
            TextFont {
                font: font.clone(),
                font_size: 40.0,
                ..Default::default()
            }
        ]
    ));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn difficulty_menu_buttons_interactions_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<(
        Entity,
        &Interaction, 
        &mut BackgroundColor,
        &mut Button
    ),
        Changed<Interaction>,
    >
) {
    for (entity, interaction, mut bg_color, mut button) in 
        interaction_query.iter_mut() {
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
pub fn hard_difficulty_button_system(
    mut game_state: ResMut<GameState>,
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>, 
            With<HardDiff>
        )
        >,
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.difficulty = Some(HARD_DIFFICULTY_DEPTH);

            if let Some(engine) = &mut game_state.engine {
                engine.set_depth(HARD_DIFFICULTY_DEPTH);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn medium_difficulty_button_system(
    mut game_state: ResMut<GameState>,
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>, 
            With<HardDiff>
        )
        >,
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.difficulty = Some(MEDIUM_DIFFICULTY_DEPTH);

            if let Some(engine) = &mut game_state.engine {
                engine.set_depth(MEDIUM_DIFFICULTY_DEPTH);
            }
        }
    }
}


#[cfg(not(target_arch = "wasm32"))]
pub fn easy_difficulty_button_system(
    mut game_state: ResMut<GameState>,
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>, 
            With<HardDiff>
        )
        >,
) {
    for interaction in interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.difficulty = Some(EASY_DIFFICULTY_DEPTH);

            if let Some(engine) = &mut game_state.engine {
                engine.set_depth(EASY_DIFFICULTY_DEPTH);
            }
        }
    }
}