use super::*;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub struct WaitingText;
#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub struct WaitingScrren;

// Component to mark the Text entity
#[derive(Component)]
pub struct NextMoveText;

// Create a marker component for the winner text
#[derive(Component)]
pub struct WinnerText;

#[derive(Component)]
pub struct GameOverScreen;
#[derive(Component)]
pub struct ReplayButton;

#[derive(Component)]
pub struct MenuScreen;
#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub struct PlayMultiplayerButton;
#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub struct PlayWithAiButton;
#[derive(Component)]
pub struct PlayOfflineButton;
#[derive(Component)]
pub struct ExitButton;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub struct DifficultyMenuScreen;
#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub struct HardDiff;
#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub struct MediumDiff;
#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub struct EasyDiff;

#[derive(Component)]
pub enum TimerPosition {
    Top,
    Bottom,
}
