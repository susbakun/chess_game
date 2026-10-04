use bevy::prelude::*;
use bevy::{ecs::relationship::RelatedSpawnerCommands, input_focus::InputFocus};

use crate::constants::*;
use crate::game_state::*;
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
#[cfg(not(target_arch = "wasm32"))]
mod difficulty_menu;
#[cfg(not(target_arch = "wasm32"))]
use difficulty_menu::*;
use shared::*;
#[cfg(not(target_arch = "wasm32"))]
mod waiting_screen;
#[cfg(not(target_arch = "wasm32"))]
use waiting_screen::*;
