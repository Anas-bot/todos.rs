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

pub fn todos_tab(todos: &mut Todos, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(RichText::new("todos.rs").size(HEADING_FONT_SIZE));
            ui.add_space(VERTICAL_SPACING);
            let text_edit = ui.add(
                TextEdit::singleline(&mut todos.new_todo)
                    .hint_text(RichText::new("What needs to be done?").size(PLACEHOLDER_FONT_SIZE))
                    .font(FontId {
                        size: PLACEHOLDER_FONT_SIZE,
                        family: Default::default(),
                    })
                    .desired_width(TEXTBOX_WIDTH)
                    .margin(vec2(16.0, 8.0)),
            );
            if text_edit.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                todos.todos.push(Todo::new(todos.new_todo.clone()));
                todos.new_todo.clear();
            }
            ui.add_space(VERTICAL_SPACING);
        });

        ui.horizontal(|ui| {
            ui.add_space(HORIZONTAL_SPACING);
            if todos.todos.len() == 1 && !todos.todos[0].checked {
                ui.label(RichText::new("1 task left").size(NORMAL_FONT_SIZE));
            } else if todos.todos.len() == 1 && todos.todos[0].checked {
                ui.label(RichText::new("0 tasks left").size(NORMAL_FONT_SIZE));
            } else {
                ui.label(
                    RichText::new(format!("{} tasks left", todos.todos.len()))
                        .size(NORMAL_FONT_SIZE),
                );
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(HORIZONTAL_SPACING);
                let completed = ui.add(
                    Button::new(RichText::new("Completed").size(NORMAL_FONT_SIZE))
                        .rounding(BUTTON_ROUNDING)
                        .fill(todos.visuals.completed_button_bg_color),
                );
                ui.add_space(SPACING_BETWEEN_BUTTONS);
                let active = ui.add(
                    Button::new(RichText::new("Active").size(NORMAL_FONT_SIZE))
                        .rounding(BUTTON_ROUNDING)
                        .fill(todos.visuals.active_button_bg_color),
                );
                ui.add_space(SPACING_BETWEEN_BUTTONS);
                let all = ui.add(
                    Button::new(RichText::new("All").size(NORMAL_FONT_SIZE))
                        .rounding(BUTTON_ROUNDING)
                        .fill(todos.visuals.all_button_bg_color),
                );

                if all.clicked() {
                    todos.filter = Filter::All;
                    todos.visuals.reset_button_color();
                    todos.visuals.all_button_bg_color = DARK_GREY;
                } else if completed.clicked() {
                    todos.filter = Filter::Completed;
                    todos.visuals.reset_button_color();
                    todos.visuals.completed_button_bg_color = DARK_GREY;
                } else if active.clicked() {
                    todos.filter = Filter::Active;
                    todos.visuals.reset_button_color();
                    todos.visuals.active_button_bg_color = DARK_GREY;
                }
            });
        });

        if !todos.todos.is_empty() {
            for (index, todo) in todos.todos.iter_mut().enumerate() {
                ui.add_space(VERTICAL_SPACING);
                ui.horizontal(|ui| {
                    ui.add_space(HORIZONTAL_SPACING);
                    if todo.is_editing {
                        let text_edit = ui.add(TextEdit::singleline(&mut todo.todo).font(FontId {
                            size: CHECKBOX_TEXT_FONT_SIZE,
                            family: Default::default(),
                        }));

                        if text_edit.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                            todo.is_editing = false;
                        }

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.add_space(HORIZONTAL_SPACING);
                            let tick_image_button = ui.add(ImageButton::new(
                                todos.visuals.tick_img_texture_handle.texture_id(ctx),
                                IMAGE_DIMENSIONS,
                            ));
                            if tick_image_button.clicked() {
                                todo.is_editing = false;
                            }
                        });
                    } else {
                        match todos.filter {
                            Filter::Completed => {
                                if todo.checked {
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
                            Filter::Active => {
                                if !todo.checked {
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

                        if (todos.filter == Filter::Completed && todo.checked)
                            || (todos.filter == Filter::Active && !todo.checked)
                            || (todos.filter == Filter::All)
                        {
                            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                ui.add_space(HORIZONTAL_SPACING);
                                let bin_image_button = ui.add(ImageButton::new(
                                    todos.visuals.bin_img_texture_handle.texture_id(ctx),
                                    IMAGE_DIMENSIONS,
                                ));
                                let edit_image_button = ui.add(ImageButton::new(
                                    todos.visuals.edit_img_texture_handle.texture_id(ctx),
                                    IMAGE_DIMENSIONS,
                                ));
                                ui.add_space(10.0);
                                if bin_image_button.clicked() {
                                    todos.to_delete_todos.push(index);
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
        if !todos.to_delete_todos.is_empty() {
            todos.remove_todo()
        }
    });
}

pub fn timer(ctx: &Context){

    CentralPanel::default().show(ctx, |ui| {
        ui.add_space(40.0);
        ui.vertical_centered(|ui|{
            ui.label(RichText::new("timer.rs").size(HEADING_FONT_SIZE));
            ui.add_space(VERTICAL_SPACING);
            ui.label(RichText::new("00:00").size(30.0));
            ui.add_space(VERTICAL_SPACING * 2.0);

            ui.horizontal(|ui|{
                let (left, right) = {
                    let rect = ui.available_rect_before_wrap();
                    let mut left_half = rect.clone();
                    left_half.set_right(rect.center().x);
                    let mut right_half = rect.clone();
                    right_half.set_left(rect.center().x);
                    (left_half, right_half)
                };

                ui.allocate_ui_at_rect(left, |ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui|{
                        ui.add_space(15.0);
                        ui.button(RichText::new("Start").size(25.0))
                    });
                });

                ui.allocate_ui_at_rect(right, |ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui|{
                        ui.add_space(15.0);
                        ui.button(RichText::new("Stop").size(25.0))
                    });
                });
            });

        });


    });

}

pub fn tabs(todos: &mut Todos, ctx: &Context){
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
                        .fill(todos.visuals.todos_button_bg_color),
                );

                let timer_tab_btn = ui.add(
                    Button::new(RichText::new("timer").size(18.0))
                        .rounding(BUTTON_ROUNDING)
                        .fill(todos.visuals.timer_button_bg_color),
                );

                if timer_tab_btn.clicked() {
                    todos.visuals.todos_button_bg_color = TRANSPARENT;
                    todos.visuals.timer_button_bg_color = DARK_GREY;
                    todos.tab = Tab::Timer;
                } else if todos_tab_btn.clicked()
                    && todos.visuals.todos_button_bg_color == TRANSPARENT
                {
                    todos.visuals.todos_button_bg_color = DARK_GREY;
                    todos.visuals.timer_button_bg_color = TRANSPARENT;
                    todos.tab = Tab::Todos
                };
            });
        });
}