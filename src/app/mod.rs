use crate::app::data_base::table::Table;
use crate::app::settings::insert_methods::{InsertMethods, InsertMethodsMessage, StartEnd};
use crate::app::settings::{MessageSettings, Settings};
use data_base::DataBase;
use iced::widget::combo_box;
use iced::Theme;
use iced_aw::date_picker;

pub mod data_base;
mod settings;
pub mod theme;
mod ui;

#[derive(Debug, Clone)]
pub enum Message {
    None,
    Create,
    SetInsertMethods(InsertMethods),
    SetInsertMethodsData(InsertMethodsMessage),
    SetData(String, usize),
    SetQuantity(String),
    SetMenu(MenuStatus),
    SetSettings(MessageSettings),
    RebootAutoInsertState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MenuStatus {
    Main,
    Settings,
}

pub struct ArchaeologicalAssistant {
    pub menu_status: MenuStatus,
    pub data: Vec<String>,
    pub quantity: String,
    pub settings: Settings,
    pub state_themes: combo_box::State<theme::Theme>,
    pub state_auto_insert: combo_box::State<u32>,
    pub is_can_start_insert: bool,
}

impl Default for ArchaeologicalAssistant {
    fn default() -> Self {
        let settings = Settings::load();
        Self {
            menu_status: MenuStatus::Main,
            data: {
                [vec![date_picker::Date::today().to_string()], {
                    if settings.path_to_db.exists() {
                        DataBase::from(&*settings.path_to_db)
                            .get_sheet()
                            .get_end_line()
                            .iter()
                            .skip(2)
                            .map(|cell| cell.get_value().to_string())
                            .collect::<Vec<_>>()
                    } else {
                        vec!["".to_string(); settings.fields.len()]
                    }
                }]
                .concat()
            },
            state_auto_insert: {
                if settings.path_to_db.exists() {
                    combo_box::State::new(DataBase::from(&*settings.path_to_db).get_sheet().skips())
                } else {
                    combo_box::State::default()
                }
            },
            quantity: "1".to_string(),
            is_can_start_insert: DataBase::from(&*settings.path_to_db)
                .get_sheet()
                .get_start_index()
                > 1.,
            settings,
            state_themes: combo_box::State::new(Vec::from(theme::Theme::ALL)),
        }
    }
}

impl ArchaeologicalAssistant {
    pub fn theme(&self) -> Theme {
        self.settings.theme.to_iced_theme()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::None => {}
            Message::SetInsertMethods(insert_methods) => {
                if insert_methods == InsertMethods::AutoInsert
                    && insert_methods != self.settings.insert_methods_data.insert_methods
                {
                    self.state_auto_insert = combo_box::State::new(
                        DataBase::from(&*self.settings.path_to_db.clone())
                            .get_sheet()
                            .skips(),
                    )
                }

                self.settings.insert_methods_data.insert_methods = insert_methods
            }
            Message::SetInsertMethodsData(insert_methods_input_types) => self
                .settings
                .insert_methods_data
                .update(insert_methods_input_types),
            Message::SetData(str, id) => self.data[id] = str,
            Message::SetQuantity(quantity) => {
                if quantity.is_empty() {
                    self.quantity = "".to_string()
                } else if quantity.parse::<u32>().is_ok() {
                    self.quantity = quantity
                }
            }
            Message::SetMenu(menu_status) => {
                if self.menu_status != menu_status {
                    if self.menu_status == MenuStatus::Settings {
                        self.settings.save()
                    } else if self.menu_status == MenuStatus::Main {
                        self.state_auto_insert = combo_box::State::new(
                            DataBase::from(&*self.settings.path_to_db.clone())
                                .get_sheet()
                                .skips(),
                        )
                    }
                }
                self.menu_status = menu_status
            }
            Message::Create => DataBase::create_record(
                &self.settings.path_to_db,
                &self.settings.print_settings,
                self.quantity.parse().unwrap_or(0),
                self.data.clone(),
                &self.settings.insert_methods_data,
            ),
            Message::SetSettings(message_settings) => self.settings.update(message_settings),
            Message::RebootAutoInsertState => {
                self.state_auto_insert = combo_box::State::new(
                    DataBase::from(&*self.settings.path_to_db.clone())
                        .get_sheet()
                        .skips(),
                )
            }
        };

        self.is_can_start_insert = if DataBase::from(&*self.settings.path_to_db)
            .get_sheet()
            .get_start_index()
            > 1.
        {
            true
        } else {
            if self.is_can_start_insert { 
                self.settings.insert_methods_data.start_end = StartEnd::End;
            }
            false
        }
    }
}
