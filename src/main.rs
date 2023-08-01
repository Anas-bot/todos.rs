use crate::app_struct::*;
use crate::constants::*;

use eframe::egui::{vec2, Context, FontId, RichText};
use eframe::{egui, run_native, Frame};
use egui::style::Spacing;
use egui::{
    Align, Button, CentralPanel, Checkbox, Color32, ImageButton, Key, Layout, Margin, TextEdit,
    TopBottomPanel,
};
use egui_extras::RetainedImage;
use crate::app::*;

mod app_struct;
mod constants;
mod app;

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(WIN_WIDTH, WIN_HEIGHT)),
        min_window_size: Some(vec2(WIN_WIDTH, WIN_HEIGHT)),
        max_window_size: Some(vec2(WIN_WIDTH, WIN_HEIGHT)),
        ..eframe::NativeOptions::default()
    };

    run_native(
        "todos.rs",
        native_options,
        Box::new(|cc| Box::new(Todos::new(cc))),
    )
    .expect("TODO: panic message");
}


impl eframe::App for Todos {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        //Call the tabs associated function to display the tabs
        tabs(self, ctx);

        match self.tab {
            Tab::Todos => {
                todos_tab(self, ctx);
            }
            Tab::Timer => {
                timer(ctx);
            }
        }
    }
}
