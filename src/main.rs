use crate::app_struct::*;
use crate::constants::*;

use eframe::egui::{vec2, Color32, Context, FontId, RichText};
use eframe::{egui, run_native, Frame};
use egui::{Align, Button, CentralPanel, Checkbox, ImageButton, Key, Layout, TextEdit};
use egui_extras::RetainedImage;

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

impl Todos {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn remove_todo(&mut self) {
        for index in &self.to_delete_todos {
            self.todos.remove(*index);
        }
        self.to_delete_todos.clear()
    }
}

impl eframe::App for Todos {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(RichText::new("todos.rs").size(HEADING_FONT_SIZE));
                ui.add_space(VERTICAL_SPACING);
                let text_edit = ui.add(
                    TextEdit::singleline(&mut self.new_todo)
                        .hint_text(
                            RichText::new("What needs to be done?").size(PLACEHOLDER_FONT_SIZE),
                        )
                        .font(FontId {
                            size: PLACEHOLDER_FONT_SIZE,
                            family: Default::default(),
                        })
                        .desired_width(TEXTBOX_WIDTH)
                        .margin(vec2(16.0, 8.0)),
                );
                if text_edit.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                    self.todos.push(Todo::new(self.new_todo.clone()));
                    self.new_todo.clear();
                }
                ui.add_space(VERTICAL_SPACING);
            });

            ui.horizontal(|ui| {
                ui.add_space(HORIZONTAL_SPACING);
                if self.todos.len() == 1 && !self.todos[0].checked {
                    ui.label(RichText::new("1 task left").size(NORMAL_FONT_SIZE));
                } else if self.todos.len() == 1 && self.todos[0].checked {
                    ui.label(RichText::new("0 tasks left").size(NORMAL_FONT_SIZE));
                } else {
                    ui.label(
                        RichText::new(format!("{} tasks left", self.todos.len()))
                            .size(NORMAL_FONT_SIZE),
                    );
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(HORIZONTAL_SPACING);
                    let completed = ui.add(
                        Button::new(RichText::new("Completed").size(NORMAL_FONT_SIZE))
                            .rounding(BUTTON_ROUNDING)
                            .fill(self.visuals.completed_button_bg_color),
                    );
                    ui.add_space(SPACING_BETWEEN_BUTTONS);
                    let active = ui.add(
                        Button::new(RichText::new("Active").size(NORMAL_FONT_SIZE))
                            .rounding(BUTTON_ROUNDING)
                            .fill(self.visuals.active_button_bg_color),
                    );
                    ui.add_space(SPACING_BETWEEN_BUTTONS);
                    let all = ui.add(
                        Button::new(RichText::new("All").size(NORMAL_FONT_SIZE))
                            .rounding(BUTTON_ROUNDING)
                            .fill(self.visuals.all_button_bg_color),
                    );

                    if all.clicked() {
                        self.filter = Filter::All;
                        self.visuals.reset_button_color();
                        self.visuals.all_button_bg_color = DARK_GREY;
                    } else if completed.clicked() {
                        self.filter = Filter::Completed;
                        self.visuals.reset_button_color();
                        self.visuals.completed_button_bg_color = DARK_GREY;
                    } else if active.clicked() {
                        self.filter = Filter::Active;
                        self.visuals.reset_button_color();
                        self.visuals.active_button_bg_color = DARK_GREY;
                    }
                });
            });

            if !self.todos.is_empty() {
                for (index, todo) in self.todos.iter_mut().enumerate() {
                    ui.add_space(VERTICAL_SPACING);
                    ui.horizontal(|ui| {
                        ui.add_space(HORIZONTAL_SPACING);
                        if todo.is_editing {
                            let text_edit =
                                ui.add(TextEdit::singleline(&mut todo.todo).font(FontId {
                                    size: CHECKBOX_TEXT_FONT_SIZE,
                                    family: Default::default(),
                                }));

                            if text_edit.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                                todo.is_editing = false;
                            }

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.add_space(HORIZONTAL_SPACING);
                                let tick_image_button = ui.add(ImageButton::new(
                                    self.visuals.tick_img_texture_handle.texture_id(ctx),
                                    IMAGE_DIMENSIONS,
                                ));
                                if tick_image_button.clicked() {
                                    todo.is_editing = false;
                                }
                            });
                        } else {
                            match self.filter {
                                Filter::Completed => {
                                    if todo.checked {
                                        ui.horizontal_wrapped(|ui| {
                                            ui.spacing_mut().icon_width = IMAGE_DIMENSIONS.x;
                                            ui.set_max_width(TEXT_MAX_WIDTH);
                                            ui.add(Checkbox::without_text(&mut todo.checked));
                                            ui.label(
                                                RichText::new(&todo.todo)
                                                    .size(CHECKBOX_TEXT_FONT_SIZE),
                                            )
                                        });
                                    }
                                }
                                Filter::Active => {
                                    if !todo.checked {
                                        ui.horizontal_wrapped(|ui| {
                                            ui.spacing_mut().icon_width = IMAGE_DIMENSIONS.x;
                                            ui.set_max_width(TEXT_MAX_WIDTH);
                                            ui.add(Checkbox::without_text(&mut todo.checked));
                                            ui.label(
                                                RichText::new(&todo.todo)
                                                    .size(CHECKBOX_TEXT_FONT_SIZE),
                                            )
                                        });
                                    }
                                }
                                Filter::All => {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.spacing_mut().icon_width = IMAGE_DIMENSIONS.x;
                                        ui.set_max_width(TEXT_MAX_WIDTH);
                                        ui.add(Checkbox::without_text(&mut todo.checked));
                                        ui.label(
                                            RichText::new(&todo.todo).size(CHECKBOX_TEXT_FONT_SIZE),
                                        )
                                    });
                                }
                            }

                            if (self.filter == Filter::Completed && todo.checked)
                                || (self.filter == Filter::Active && !todo.checked)
                                || (self.filter == Filter::All)
                            {
                                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                    ui.add_space(HORIZONTAL_SPACING);
                                    let bin_image_button = ui.add(ImageButton::new(
                                        self.visuals.bin_img_texture_handle.texture_id(ctx),
                                        IMAGE_DIMENSIONS,
                                    ));
                                    let edit_image_button = ui.add(ImageButton::new(
                                        self.visuals.edit_img_texture_handle.texture_id(ctx),
                                        IMAGE_DIMENSIONS,
                                    ));
                                    ui.add_space(10.0);
                                    if bin_image_button.clicked() {
                                        self.to_delete_todos.push(index);
                                    }
                                    if edit_image_button.clicked() {
                                        todo.is_editing = true;
                                    }
                                });
                            }
                        }
                    });
                }
            }
            if !self.to_delete_todos.is_empty() {
                self.remove_todo()
            }
        });
    }
}
