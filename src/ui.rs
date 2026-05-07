use bevy::prelude::*;
use crate::pieces::*;
use crate::player::Player;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_next_move_text)
            .add_systems(Update, next_move_text_update
                .run_if(resource_changed::<Player>));
    }
}
// Component to mark the Text entity
#[derive(Component)]
struct NextMoveText;


/// Initialize UiCamera and text
fn init_next_move_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(
        "fonts/FiraSans-Bold.ttf"
    );

    commands.spawn(Node {
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
            TextColor(Color::linear_rgb(
                0.8, 
                0.8, 
                0.8)),
            NextMoveText
        ));
    });
}


/// Update text with the correct turn
fn next_move_text_update(
    turn: Res<Player>,
    mut query: Query<(&mut Text, &NextMoveText)>
) {
    for (mut text, _tag) in query.iter_mut() {
        text.0 = format!(
            "Next move: {}",
            match turn.0 {
                PieceColor::White => "White",
                PieceColor::Black => "Black"
            }
        )
    }
}
