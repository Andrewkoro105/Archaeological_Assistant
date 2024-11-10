use crate::app::Message;
use crate::theme::Theme;
use ciborium::from_reader;
use ciborium::into_writer;
use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use sugar::hashmap;

#[derive(Debug, Clone)]
pub enum Axis {
    X,
    Y,
}

#[derive(Serialize, Deserialize)]
pub enum FieldType {
    Text,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    pub field_type: FieldType,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct InputNumberForPrintSettings {
    pub height_text: String,
    pub pos_image: (String, String),
    pub size: (String, String),
    pub size_image: (String, String),
    pub text_size: String,
}

#[derive(Serialize, Deserialize)]
pub struct PrintSettings {
    pub font: Box<Path>,
    pub input_number: InputNumberForPrintSettings,
    height_text: u32,
    pos_image: (u32, u32),
    size: (u32, u32),
    size_image: (u32, u32),
    text_size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub current_language: String,
    pub fields: Vec<Field>,
    pub path_to_db: Box<Path>,
    pub print_settings: PrintSettings,
    pub theme: Theme,
}

struct InputSettings {}

impl Field {
    pub fn new(field_type: FieldType, name: String) -> Self {
        Self { field_type, name }
    }
}

impl Default for PrintSettings {
    fn default() -> Self {
        Self {
            font: Path::new(&format!(
                "{}/Archaeological_assistant/18685.ttf",
                env::var("HOME").unwrap()
            ))
            .into(),
            input_number: Default::default(),
            height_text: 1,
            pos_image: (10, 33),
            size: (40, 58),
            size_image: (20, 20),
            text_size: 15,
        }
    }
}

impl Default for InputNumberForPrintSettings {
    fn default() -> Self {
        Self {
            height_text: "1".to_string(),
            pos_image: ("10".to_string(), "33".to_string()),
            size: ("40".to_string(), "58".to_string()),
            size_image: ("20".to_string(), "20".to_string()),
            text_size: "15".to_string(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            current_language: "ru".to_string(),
            fields: vec![
                Field::new(FieldType::Text, "Name".to_string()),
                Field::new(FieldType::Text, "Square".to_string()),
                Field::new(FieldType::Text, "Tomb".to_string()),
                Field::new(FieldType::Text, "info".to_string()),
            ],
            path_to_db: Path::new(&format!(
                "{}/Archaeological_assistant/Archaeological_assistant_db.xlsx",
                env::var("HOME").unwrap()
            ))
            .into(),
            print_settings: PrintSettings::default(),
            theme: Theme::Dark,
        }
    }
}

impl Settings {
    fn select_file<TextInputMessage>(
        &mut self,
        message: TextInputMessage,
        file_types: Vec<(&'static str, &'static [&'static str])>,
    ) where
        TextInputMessage: Fn(String) -> Message + Clone,
    {
        let mut dialog = FileDialog::new().set_location("~/Desktop");
        for (file_type, extensions) in file_types {
            dialog = dialog.clone().add_filter(file_type, extensions);
        }
        let path = dialog
            .show_open_single_file()
            .map(|opt_path| opt_path.map(|path| path.to_str().map(|path| path.to_string())));

        if let Ok(Some(Some(str))) = path {
            self.update(message(str));
        }
    }

    fn set_number_settings(str: &String, input: &mut String, num: &mut u32) {
        if str.is_empty() {
            *input = "".to_string();
            *num = 0;
        } else if let Ok(new_num) = str.parse::<u32>() {
            *input = str.clone();
            *num = new_num;
        }
    }

    fn set_number_with_coordinate_settings(
        str: &String,
        input: &mut (String, String),
        num: &mut (u32, u32),
        axis: Axis,
    ) {
        match axis {
            Axis::X => Self::set_number_settings(str, &mut input.0, &mut num.0),
            Axis::Y => Self::set_number_settings(str, &mut input.1, &mut num.1),
        }
    }

    pub fn update(&mut self, message: Message) -> Message {
        match message {
            Message::SelectDb(file_types) => {
                self.select_file(Message::SetPathToDb, file_types);
                Message::None
            }
            Message::SelectFont(file_types) => {
                self.select_file(Message::SetPathToFont, file_types);
                Message::None
            }
            Message::SetPathToDb(path_str) => {
                self.path_to_db = Box::from(Path::new(&path_str));
                Message::None
            }
            Message::SetPathToFont(path_str) => {
                self.print_settings.font = Box::from(Path::new(&path_str));
                Message::None
            }
            Message::SetSize(size, axis) => {
                Self::set_number_with_coordinate_settings(
                    &size,
                    &mut self.print_settings.input_number.size,
                    &mut self.print_settings.size,
                    axis,
                );
                Message::None
            }
            Message::SetSizeImage(size_image, axis) => {
                Self::set_number_with_coordinate_settings(
                    &size_image,
                    &mut self.print_settings.input_number.size_image,
                    &mut self.print_settings.size_image,
                    axis,
                );
                Message::None
            }
            Message::SetPositionImage(pos_image, axis) => {
                Self::set_number_with_coordinate_settings(
                    &pos_image,
                    &mut self.print_settings.input_number.pos_image,
                    &mut self.print_settings.pos_image,
                    axis,
                );
                Message::None
            }
            Message::SetHeightText(height_text) => {
                Self::set_number_settings(
                    &height_text,
                    &mut self.print_settings.input_number.height_text,
                    &mut self.print_settings.height_text,
                );
                Message::None
            }
            Message::SetTextSize(text_size) => {
                Self::set_number_settings(
                    &text_size,
                    &mut self.print_settings.input_number.text_size,
                    &mut self.print_settings.text_size,
                );
                Message::None
            }
            Message::SetTheme(theme) => {
                self.theme = theme;
                Message::None
            }
            _ => message,
        }
    }

    pub fn load() -> Self {
        let str_path = &format!(
            "{}/Archaeological_assistant/settings.cbor",
            env::var("HOME").unwrap()
        );
        let path = Path::new(str_path);
        if path.exists() {
            from_reader(std::fs::File::open(path).expect("can`t open settings.cbor"))
                .unwrap_or(Self::default())
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        into_writer(
            &self,
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(&format!(
                    "{}/Archaeological_assistant/settings.cbor",
                    env::var("HOME").unwrap()
                ))
                .expect("can`t open or create settings.cbor"),
        )
        .expect("can`t write settings.cbor");
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        self.save();
    }
}
