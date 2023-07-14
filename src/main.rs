use eframe::egui::RichText;
use eframe::egui::{vec2, Color32, Context, FontId};
use eframe::{egui, run_native, Frame};
use egui_extras::RetainedImage;

const WIN_WIDTH: f32 = 390.0;
const WIN_HEIGHT: f32 = 550.0;

const HEADING_FONT_SIZE: f32 = 50.0;
const NORMAL_FONT_SIZE: f32 = 15.0;
const PLACEHOLDER_FONT_SIZE: f32 = 25.0;
const CHECKBOX_TEXT_FONT_SIZE: f32 = 18.0;

const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);
const DARK_GREY: Color32 = Color32::from_rgb(96, 96, 96);

const TEXTBOX_WIDTH: f32 = 350.0;

const VERTICAL_SPACING: f32 = 20.0;
const HORIZONTAL_SPACING: f32 = 20.0;

const SPACING_BETWEEN_BUTTONS: f32 = 12.0;
const BUTTON_ROUNDING: f32 = 4.0;

const IMAGE_DIMENSIONS: egui::Vec2 = vec2(15.0, 15.0);

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Option::from(vec2(WIN_WIDTH, WIN_HEIGHT)),
        min_window_size: Option::from(vec2(WIN_WIDTH, WIN_HEIGHT)),
        max_window_size: Option::from(vec2(WIN_WIDTH, WIN_HEIGHT)),
        ..eframe::NativeOptions::default()
    };
    run_native(
        "todos.rs",
        native_options,
        Box::new(|cc| Box::new(Todos::new(cc))),
    )
    .expect("TODO: panic message");
}
struct Visuals {
    all_button_bg_color: Color32,
    completed_button_bg_color: Color32,
    active_button_bg_color: Color32,
    bin_img_texture_handle: RetainedImage,
}

impl Visuals {
    fn reset_button_color(&mut self) {
        self.active_button_bg_color = TRANSPARENT;
        self.all_button_bg_color = TRANSPARENT;
        self.completed_button_bg_color = TRANSPARENT;
    }
}

struct Todos {
    todos: Vec<(String, bool)>,
    new_todo: String,
    filter: Filter,
    visuals: Visuals,
    to_delete_todos: Vec<usize>,
}

#[derive(Default, PartialEq)]
enum Filter {
    #[default]
    All,
    Active,
    Completed,
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

impl Default for Todos {
    fn default() -> Self {
        Todos {
            todos: vec![],
            new_todo: "".to_string(),
            filter: Default::default(),
            visuals: Visuals {
                all_button_bg_color: TRANSPARENT,
                completed_button_bg_color: TRANSPARENT,
                active_button_bg_color: TRANSPARENT,
                bin_img_texture_handle: RetainedImage::from_image_bytes(
                    "bin",
                    include_bytes!("bin.png"),
                )
                .expect("reasons"),
            },
            to_delete_todos: vec![],
        }
    }
}

impl eframe::App for Todos {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(RichText::new("todos.rs").size(HEADING_FONT_SIZE));
                ui.add_space(VERTICAL_SPACING);
                let text_edit = ui.add(
                    egui::TextEdit::singleline(&mut self.new_todo)
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
                if text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.todos.push((self.new_todo.clone(), false));
                    self.new_todo.clear();
                }
                ui.add_space(VERTICAL_SPACING);
            });

            ui.horizontal(|ui| {
                ui.add_space(HORIZONTAL_SPACING);
                if self.todos.len() == 1 && !self.todos[0].1 {
                    ui.label(RichText::new("1 task left").size(NORMAL_FONT_SIZE));
                } else if self.todos.len() == 1 && self.todos[0].1 {
                    ui.label(RichText::new("0 tasks left").size(NORMAL_FONT_SIZE));
                } else {
                    ui.label(
                        RichText::new(format!("{} tasks left", self.todos.len()))
                            .size(NORMAL_FONT_SIZE),
                    );
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(HORIZONTAL_SPACING);
                    let completed = ui.add(
                        egui::Button::new(RichText::new("Completed").size(NORMAL_FONT_SIZE))
                            .rounding(BUTTON_ROUNDING)
                            .fill(self.visuals.completed_button_bg_color),
                    );
                    ui.add_space(SPACING_BETWEEN_BUTTONS);
                    let active = ui.add(
                        egui::Button::new(RichText::new("Active").size(NORMAL_FONT_SIZE))
                            .rounding(BUTTON_ROUNDING)
                            .fill(self.visuals.active_button_bg_color),
                    );
                    ui.add_space(SPACING_BETWEEN_BUTTONS);
                    let all = ui.add(
                        egui::Button::new(RichText::new("All").size(NORMAL_FONT_SIZE))
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
                        match self.filter {
                            Filter::Completed => {
                                if todo.1 {
                                    ui.add(egui::Checkbox::new(
                                        &mut todo.1,
                                        RichText::new(&todo.0).size(CHECKBOX_TEXT_FONT_SIZE),
                                    ));
                                }
                            }
                            Filter::Active => {
                                if !todo.1 {
                                    ui.add(egui::Checkbox::new(
                                        &mut todo.1,
                                        RichText::new(&todo.0).size(CHECKBOX_TEXT_FONT_SIZE),
                                    ));
                                }
                            }
                            Filter::All => {
                                ui.add(egui::Checkbox::new(
                                    &mut todo.1,
                                    RichText::new(&todo.0).size(CHECKBOX_TEXT_FONT_SIZE),
                                ));
                            }
                        }

                        if (self.filter == Filter::Completed && todo.1)
                            || (self.filter == Filter::Active && !todo.1)
                            || (self.filter == Filter::All)
                        {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.add_space(HORIZONTAL_SPACING);
                                    let bin_image_button = ui.add(egui::ImageButton::new(
                                        self.visuals.bin_img_texture_handle.texture_id(ctx),
                                        IMAGE_DIMENSIONS,
                                    ));
                                    if bin_image_button.clicked() {
                                        self.to_delete_todos.push(index);
                                    }
                                },
                            );
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
