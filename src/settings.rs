use crate::app::Message;
use ciborium::from_reader;
use ciborium::into_writer;
use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::path::Path;
use crate::theme::Theme;
use std::env;

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
pub struct PrintSettings {
    pub font: Box<Path>,
    pub height_text: u32,
    pub pos_image: (u32, u32),
    pub size: (u32, u32),
    pub size_image: (u32, u32),
    pub text_size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub current_language: String,
    pub fields: Vec<Field>,
    pub path_to_db: Box<Path>,
    pub print_settings: PrintSettings,
    pub theme: Theme,
}

impl Field {
    pub fn new(field_type: FieldType, name: String) -> Self {
        Self { field_type, name }
    }
}

impl Default for PrintSettings {
    fn default() -> Self {
        Self {
            font: Path::new(&format!("{}/Archaeological_assistant/18685.ttf", env::var("HOME").unwrap())).into(),
            height_text: 1,
            pos_image: (10, 33),
            size: (40, 58),
            size_image: (20, 20),
            text_size: 15,
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
            path_to_db: Path::new(
                &format!("{}/Archaeological_assistant/Archaeological_assistant_db.xlsx", env::var("HOME").unwrap()),
            )
            .into(),
            print_settings: PrintSettings::default(),
            theme: Theme::Dark
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
            Message::SetSize(size, axis) => match axis {
                Axis::X => {
                    self.print_settings.size.0 = size.parse().unwrap_or(self.print_settings.size.0);
                    Message::None
                }
                Axis::Y => {
                    self.print_settings.size.1 = size.parse().unwrap_or(self.print_settings.size.1);
                    Message::None
                }
            },
            Message::SetSizeImage(size_image, axis) => match axis {
                Axis::X => {
                    self.print_settings.size_image.0 = size_image
                        .parse()
                        .unwrap_or(self.print_settings.size_image.0);
                    Message::None
                }
                Axis::Y => {
                    self.print_settings.size_image.1 = size_image
                        .parse()
                        .unwrap_or(self.print_settings.size_image.1);
                    Message::None
                }
            },
            Message::SetPositionImage(pos_image, axis) => match axis {
                Axis::X => {
                    self.print_settings.pos_image.0 =
                        pos_image.parse().unwrap_or(self.print_settings.pos_image.0);
                    Message::None
                }
                Axis::Y => {
                    self.print_settings.pos_image.1 =
                        pos_image.parse().unwrap_or(self.print_settings.pos_image.1);
                    Message::None
                }
            },
            Message::SetHeightText(height_text) => {
                self.print_settings.height_text = height_text
                    .parse()
                    .unwrap_or(self.print_settings.height_text);
                Message::None
            }
            Message::SetTextSize(text_size) => {
                self.print_settings.text_size =
                    text_size.parse().unwrap_or(self.print_settings.text_size);
                Message::None
            }
            Message::SetTheme(id, _) => { 
                self.theme = id.into();
                Message::None
            }
            _ => message,
        }
    }

    pub fn load() -> Self {
        let str_path = &format!("{}/Archaeological_assistant/settings.cbor", env::var("HOME").unwrap());
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
                .open(&format!("{}/Archaeological_assistant/settings.cbor", env::var("HOME").unwrap()))
                .expect("can`t open or create settings.cbor"),
        )
        .expect("can`t write settings.cbor");
    }
}
