use bevy::color::Color;

// colors
pub const NORMAL_END_MENU_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_END_MENU_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_END_MENU_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
pub const NORMAL_START_MENU_BUTTON: Color = Color::linear_rgb(0.0, 0.1, 0.1);
pub const HOVERED_START_MENU_BUTTON: Color = Color::linear_rgb(0.25, 0.6, 0.9);
pub const PRESSED_START_MENU_BUTTON: Color = Color::linear_rgb(0.15, 0.4, 0.7);

// difficulty
pub const HARD_DIFFICULTY_DEPTH: u32 = 10;
pub const MEDIUM_DIFFICULTY_DEPTH: u32 = 8;
pub const EASY_DIFFICULTY_DEPTH: u32 = 6;
