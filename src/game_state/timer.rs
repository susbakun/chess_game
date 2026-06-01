use crate::pieces::PieceColor;

use super::*;

pub fn change_timer(time: Res<Time>, mut tick: Local<Timer>, mut game_state: ResMut<GameState>) {
    if game_state.game_type == Some(GameType::PlayWithAi) {
        return;
    }

    if tick.duration().is_zero() {
        *tick = Timer::from_seconds(1.0, TimerMode::Repeating);
    }

    if game_state.game_over || game_state.game_type.is_none() {
        return;
    }

    let player_color = game_state.player.0;

    tick.tick(time.delta());

    if !tick.is_finished() {
        return;
    }

    match player_color {
        PieceColor::White => {
            if game_state.timer.0 == 0 {
                game_state.game_over = true;
                game_state.winner = Some(PieceColor::Black);
            } else {
                game_state.timer.0 = game_state.timer.0.saturating_sub(1)
            }
        }
        PieceColor::Black => {
            if game_state.timer.1 == 0 {
                game_state.game_over = true;
                game_state.winner = Some(PieceColor::White);
            } else {
                game_state.timer.1 = game_state.timer.1.saturating_sub(1)
            }
        }
    }
}
