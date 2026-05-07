use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod spawn;
mod utils;

pub use components::*;
pub use plugins::*;
pub use resources::*;
pub use spawn::*;
pub use utils::*;



#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn
}

pub fn create_pieces(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    piece_handles: Res<PieceHandles>
) {
    let white_material = materials.add(
        Color::linear_rgb(1.0, 0.9, 0.9));

    let black_material = materials.add(
        Color::linear_rgb(0.0, 0.1, 0.1)
    );

    // white pieces
    spawn_piece(
        commands.reborrow(), 
        white_material.clone(), 
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Rook, 
            x: 0, 
            y: 0,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Knight, 
            x: 0, 
            y: 1,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Bishop, 
            x: 0, 
            y: 2,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Queen, 
            x: 0,
            y: 3,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::King, 
            x: 0, 
            y: 4,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Bishop, 
            x: 0, 
            y: 5,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Knight, 
            x: 0, 
            y: 6,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(), 
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Rook, 
            x: 0, 
            y: 7,
            taken: false
        },
        piece_handles.clone()
    );

    for i in 0..8 {
        spawn_piece(
            commands.reborrow(), 
            white_material.clone(),
            Piece { 
                color: PieceColor::White, 
                piece_type: PieceType::Pawn,
                x: 1, 
                y: i,
                taken: false
            },
            piece_handles.clone()
        );
    }


    // black pieces
    spawn_piece(
        commands.reborrow(), 
        black_material.clone(), 
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Rook, 
            x: 7,
            y: 0,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Knight, 
            x: 7, 
            y: 1,
            taken: false 
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Bishop, 
            x: 7, 
            y: 2,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Queen, 
            x: 7, 
            y: 3,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::King, 
            x: 7, 
            y: 4,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Bishop, 
            x: 7, 
            y: 5,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Knight, 
            x: 7, 
            y: 6,
            taken: false
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(), 
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Rook, 
            x: 7, 
            y: 7,
            taken: false
        },
        piece_handles.clone()
    );

    for i in 0..8 {
        spawn_piece(
            commands.reborrow(), 
            black_material.clone(),
            Piece { 
                color: PieceColor::Black, 
                piece_type: PieceType::Pawn,
                x: 6, 
                y: i,
                taken: false
            },
            piece_handles.clone()
        );
    }
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        let direction = vec3(piece.x as f32, 0.0, piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += 
                direction.normalize() * 
                time.delta_secs() * 
                vec3(2.0, 2.0, 2.0);
        }
    }
}
