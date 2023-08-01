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
        TopBottomPanel::top("tabs")
            .show_separator_line(true)
            .frame(egui::containers::Frame {
                outer_margin: Margin {
                    left: 0.0,
                    ..Margin::default()
                },
                fill: Color32::from_rgba_premultiplied(27, 27, 27, 255),
                ..egui::containers::Frame::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                    let todos_tab_btn = ui.add(
                        Button::new(RichText::new("todos").size(18.0))
                            .rounding(BUTTON_ROUNDING)
                            .fill(self.visuals.todos_button_bg_color),
                    );

                    let timer_tab_btn = ui.add(
                        Button::new(RichText::new("timer").size(18.0))
                            .rounding(BUTTON_ROUNDING)
                            .fill(self.visuals.timer_button_bg_color),
                    );

                    if timer_tab_btn.clicked() {
                        self.visuals.todos_button_bg_color = TRANSPARENT;
                        self.visuals.timer_button_bg_color = DARK_GREY;
                        self.tab = Tab::Timer;
                    } else if todos_tab_btn.clicked()
                        && self.visuals.todos_button_bg_color == TRANSPARENT
                    {
                        self.visuals.todos_button_bg_color = DARK_GREY;
                        self.visuals.timer_button_bg_color = TRANSPARENT;
                        self.tab = Tab::Todos
                    };
                });
            });
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
