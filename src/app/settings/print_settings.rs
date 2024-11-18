use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

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
    pub(crate) height_text: u32,
    pub pos_image: (u32, u32),
    pub size: (u32, u32),
    pub size_image: (u32, u32),
    pub text_size: u32,
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
