use super::*;

#[derive(Component)]
pub struct Square {
    pub x: i8,
    pub y: i8
}

impl Square {
    pub fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}


#[derive(Component)]
pub struct Taken;
