use bevy::prelude::*;
use crate::pieces::*;
use crate::player::*;

mod next_move_text;
use next_move_text::*;
mod components;
use components::*;
mod winner_text;
use winner_text::*;
mod plugins;
pub use plugins::*;

use crate::GameOver;