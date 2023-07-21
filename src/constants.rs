use eframe::egui::vec2;
use eframe::egui::Color32;

pub const WIN_WIDTH: f32 = 390.0;
pub const WIN_HEIGHT: f32 = 550.0;

pub const HEADING_FONT_SIZE: f32 = 50.0;
pub const NORMAL_FONT_SIZE: f32 = 15.0;
pub const PLACEHOLDER_FONT_SIZE: f32 = 25.0;
pub const CHECKBOX_TEXT_FONT_SIZE: f32 = 18.0;

pub const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);
pub const DARK_GREY: Color32 = Color32::from_rgb(96, 96, 96);

pub const TEXTBOX_WIDTH: f32 = 350.0;

pub const VERTICAL_SPACING: f32 = 20.0;
pub const HORIZONTAL_SPACING: f32 = 20.0;

pub const SPACING_BETWEEN_BUTTONS: f32 = 12.0;
pub const BUTTON_ROUNDING: f32 = 4.0;

pub const IMAGE_DIMENSIONS: egui::Vec2 = vec2(16.0, 16.0);
pub const TEXT_MAX_WIDTH: f32 = 280.0;
