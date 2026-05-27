use super::*;

#[derive(Resource)]
pub struct SpacetimeConnection {
    pub conn: DbConnection,
}
