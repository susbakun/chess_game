use bevy::prelude::*;
use bevy::{ecs::relationship::RelatedSpawnerCommands, input_focus::InputFocus};

use crate::constants::*;
use crate::game_state::*;
use crate::pieces::*;
use crate::replay::*;

mod play_info;
use play_info::*;
mod components;
use components::*;
mod end_menu;
use end_menu::*;
mod plugins;
pub use plugins::*;
mod start_menu;
pub use start_menu::*;
mod difficulty_menu;
use difficulty_menu::*;
