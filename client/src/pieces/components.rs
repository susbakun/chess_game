use super::*;

#[derive(Component, Clone, Copy, Debug, Deref, DerefMut)]
pub struct PieceComponent(pub Piece);
