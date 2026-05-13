use bevy::prelude::*;
use bevy::{
    ecs::relationship::RelatedSpawnerCommands, 
    input_focus::InputFocus
};


use crate::pieces::*;
use crate::constants::*;
use crate::game_state::*;
use crate::replay::*;

mod next_move_text;
use next_move_text::*;
mod components;
use components::*;
mod end_menu;
use end_menu::*;
mod plugins;
pub use plugins::*;
mod start_menu;
pub use start_menu::*;