use crate::constants::{DARK_GREY, TRANSPARENT};
use eframe::egui::Color32;
use egui::accesskit::Role::Time;
use egui::Style;
use egui_extras::RetainedImage;
use std::time;

pub struct Todo {
    pub todo: String,
    pub checked: bool,
    pub is_editing: bool,
}

impl Todo {
    pub fn new(todo: String) -> Self {
        Todo {
            todo,
            checked: false,
            is_editing: false,
        }
    }
}

pub struct Visuals {
    pub all_button_bg_color: Color32,
    pub completed_button_bg_color: Color32,
    pub active_button_bg_color: Color32,
    pub todos_button_bg_color: Color32,
    pub timer_button_bg_color: Color32,
    pub bin_img_texture_handle: RetainedImage,
    pub edit_img_texture_handle: RetainedImage,
    pub tick_img_texture_handle: RetainedImage,
}

impl Visuals {
    pub(crate) fn reset_button_color(&mut self) {
        self.active_button_bg_color = TRANSPARENT;
        self.all_button_bg_color = TRANSPARENT;
        self.completed_button_bg_color = TRANSPARENT;
    }
}

pub struct Todos {
    pub todos: Vec<Todo>,
    pub new_todo: String,
    pub filter: Filter,
    pub visuals: Visuals,
    pub to_delete_todos: Vec<usize>,
    pub tab: Tab,
}

impl Todos {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn remove_todo(&mut self) {
        for index in &self.to_delete_todos {
            self.todos.remove(*index);
        }
        self.to_delete_todos.clear()
    }
}


struct Timer {
    duration: String,
    pause_at: Option<String>,
}

impl Timer {
    fn new(duration: String) -> Self {
        Timer {
            duration,
            pause_at: None,
        }
    }
}

#[derive(Default, PartialEq)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}
#[derive(Default)]
pub enum Tab {
    #[default]
    Todos,
    Timer,
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
                todos_button_bg_color: DARK_GREY,
                timer_button_bg_color: TRANSPARENT,
                bin_img_texture_handle: RetainedImage::from_image_bytes(
                    "bin",
                    include_bytes!("media/bin.png"),
                )
                .expect("reasons"),
                edit_img_texture_handle: RetainedImage::from_image_bytes(
                    "edit",
                    include_bytes!("media/edit.png"),
                )
                .expect("reasons"),
                tick_img_texture_handle: RetainedImage::from_image_bytes(
                    "edit",
                    include_bytes!("media/check.png"),
                )
                .expect("reasons"),
            },
            to_delete_todos: vec![],
            tab: Default::default(),
        }
    }
}
