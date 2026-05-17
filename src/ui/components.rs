use super::*;

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
#[derive(Component)]
pub struct PlayWithAiButton;
#[derive(Component)]
pub struct PlayOfflineButton;
#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct DifficultyMenuScreen;
#[derive(Component)]
pub struct HardDiff;
#[derive(Component)]
pub struct MediumDiff;
#[derive(Component)]
pub struct EasyDiff;
