use bevy::prelude::*;
use crate::pieces::*;
use crate::player::*;
use crate::constants::*;

mod next_move_text;
use next_move_text::*;
mod components;
use components::*;
mod winner_text;
use winner_text::*;
mod plugins;
pub use plugins::*;
mod replay_button;
pub use replay_button::*;


use crate::GameOver;