use super::*;

#[derive(Resource, Clone)]
pub struct PieceHandles {
    pub king_handle: Handle<Mesh>,
    pub king_cross_handle: Handle<Mesh>,
    pub queen_handle: Handle<Mesh>,
    pub knight_1_handle: Handle<Mesh>,
    pub knight_2_handle: Handle<Mesh>,
    pub bishop_handle: Handle<Mesh>,
    pub rook_handle: Handle<Mesh>,
    pub pawn_handle: Handle<Mesh>
}

impl FromWorld for PieceHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let king_handle: Handle<Mesh> = 
            asset_server.load("models/pieces.glb#Mesh0/Primitive0");
        let king_cross_handle: Handle<Mesh> = 
            asset_server.load("models/pieces.glb#Mesh1/Primitive0");
        let pawn_handle: Handle<Mesh> = 
            asset_server.load("models/pieces.glb#Mesh2/Primitive0");
        let knight_1_handle: Handle<Mesh> = 
            asset_server.load("models/pieces.glb#Mesh3/Primitive0");
        let knight_2_handle: Handle<Mesh> =
                asset_server.load("models/pieces.glb#Mesh4/Primitive0");
        let rook_handle: Handle<Mesh> =
                asset_server.load("models/pieces.glb#Mesh5/Primitive0");
        let bishop_handle: Handle<Mesh> =
                asset_server.load("models/pieces.glb#Mesh6/Primitive0");
        let queen_handle: Handle<Mesh> =
                asset_server.load("models/pieces.glb#Mesh7/Primitive0");

        Self {
            king_handle,
            king_cross_handle,
            queen_handle,
            knight_1_handle,
            knight_2_handle,
            rook_handle,
            bishop_handle,
            pawn_handle
        }
    }
}
