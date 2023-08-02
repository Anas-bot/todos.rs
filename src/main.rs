use crate::app_struct::*;
use crate::constants::*;

use crate::app::*;
use eframe::egui::{vec2, Context};
use eframe::{egui, run_native, Frame};

mod app;
mod app_struct;
mod constants;

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
        //call the tabs associated function to display the tabs

        //I create an instance everytime I loop :c
        // I think I should just add the timer struct to the main app struct

        tabs(self, ctx);

        match self.tab {
            Tab::Todos => {
                todos_tab(self, ctx);
            }
            Tab::Timer => {

                timer(self, ctx);
            }
        }
    }
}
