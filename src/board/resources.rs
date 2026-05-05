use super::*;


#[derive(Resource)]
pub struct PlayerTurn(pub PieceColor);

impl PlayerTurn {
    pub fn change(&mut self) {
        self.0 = match self.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White
        }
    }
}

impl Default for PlayerTurn {
    fn default() -> Self {
        PlayerTurn(PieceColor::White)
    }
}

#[derive(Default, Resource)]
pub struct SelectedSquare {
    pub entity: Option<Entity>
}

#[derive(Default, Resource)]
pub struct SelectedPiece {
    pub entity: Option<Entity>
}

#[derive(Resource)]
pub struct SquareMaterials {
    pub highlight_color: Handle<StandardMaterial>,
    pub selected_color: Handle<StandardMaterial>,
    pub black_color: Handle<StandardMaterial>,
    pub white_color: Handle<StandardMaterial>,
}

impl FromWorld for SquareMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = 
            world.resource_mut::<Assets<StandardMaterial>>();

        Self {
            highlight_color: materials.add(
                Color::linear_rgb(0.8, 0.3, 0.3)
            ),
            selected_color: materials.add(
                Color::linear_rgb(0.9, 0.1, 0.1)
            ),
            black_color: materials.add(
                Color::linear_rgb(0., 0.1, 0.1)
            ),
            white_color: materials.add(
                Color::linear_rgb(1., 0.9, 0.9)
            ),
        }
    }
}


// counting removed pieces (whites, blacks)
#[derive(Resource, Default)]
pub struct GoneCount(pub u8, pub u8);
