use iced::widget::combo_box;
use serde::{Deserialize, Serialize};
use crate::app::data_base::DataBase;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum InsertMethods {
    StartEnd,
    Input,
    AutoInsert,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum InsertMethodsMessage {
    StartEnd(StartEnd),
    Input(String),
    AutoInsert(u32),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StartEnd {
    Start,
    End,
}

#[derive(Serialize, Deserialize)]
pub struct InputNumberForInsertMethodsData {
    pub input: String,
    pub auto_insert: String,
}

#[derive(Serialize, Deserialize)]
pub struct InsertMethodsData {
    pub input_number_for_insert_methods_data: InputNumberForInsertMethodsData,
    pub insert_methods: InsertMethods,
    pub start_end: StartEnd,
    pub input: u32,
    pub auto_insert: u32,
}

impl InsertMethodsData {
    fn set_option_number_settings(str: &String, input: &mut String, num: &mut u32) {
        if str.is_empty() {
            *input = "".to_string();
            *num = 1;
        } else if let Ok(new_num) = str.parse::<u32>() {
            *input = str.clone();
            *num = new_num;
        }
    }

    pub fn update(&mut self, message: InsertMethodsMessage) {
        match message {
            InsertMethodsMessage::StartEnd(start_end) => {
                self.start_end = start_end;
            }
            InsertMethodsMessage::Input(input) => Self::set_option_number_settings(
                &input,
                &mut self.input_number_for_insert_methods_data.input,
                &mut self.input,
            ),
            InsertMethodsMessage::AutoInsert(auto_insert) => {
                self.auto_insert = auto_insert 
            },
        }
    }
}

impl Default for InputNumberForInsertMethodsData {
    fn default() -> Self {
        Self {
            input: "1".to_string(),
            auto_insert: "".to_string()
        }
    }
}

impl Default for InsertMethodsData {
    fn default() -> Self {
        Self {
            input_number_for_insert_methods_data: Default::default(),
            insert_methods: InsertMethods::StartEnd,
            start_end: StartEnd::End,
            input: 1,
            auto_insert: 0,
        }
    }
}
