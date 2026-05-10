use super::*;

#[derive(Resource)]
pub struct GameOver(pub bool);

impl Default for GameOver {
    fn default() -> Self {
        Self(false)
    }
}

impl GameOver {
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
}