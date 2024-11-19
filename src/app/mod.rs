use crate::app::data_base::table::Table;
use crate::app::settings::insert_methods::{InsertMethods, InsertMethodsMessage, StartEnd};
use crate::app::settings::{MessageSettings, Settings};
use data_base::DataBase;
use iced::font::{Family, Stretch, Weight};
use iced::widget::combo_box;
use iced::{window, Font, Subscription, Task, Theme};
use iced_aw::date_picker;

pub mod data_base;
mod settings;
pub mod theme;
mod ui;

#[derive(Debug, Clone)]
pub enum Message {
    None,
    Create,
    Update,
    Terminal(iced_term::Event),
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
    Update,
}

pub struct ArchaeologicalAssistant {
    pub menu_status: MenuStatus,
    pub data: Vec<String>,
    pub quantity: String,
    pub settings: Settings,
    pub state_themes: combo_box::State<theme::Theme>,
    pub state_auto_insert: combo_box::State<u32>,
    pub is_can_start_insert: bool,
    term: iced_term::Terminal,
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
            term: iced_term::Terminal::new(
                0,
                iced_term::settings::Settings {
                    font: iced_term::settings::FontSettings {
                        size: 14.0,
                        font_type: Font {
                            weight: Weight::Bold,
                            family: Family::Name("JetBrainsMono Nerd Font Mono"),
                            stretch: Stretch::Normal,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    theme: iced_term::settings::ThemeSettings::default(),
                    backend: iced_term::settings::BackendSettings {
                        shell: std::env::var("SHELL")
                            .expect("SHELL variable is not defined")
                            .to_string(),
                        input: false,
                    },
                },
            ),
        }
    }
}

impl ArchaeologicalAssistant {
    pub fn theme(&self) -> Theme {
        self.settings.theme.to_iced_theme()
    }

    pub(crate) fn subscription(&self) -> Subscription<Message> {
        let term_subscription = iced_term::Subscription::new(self.term.id);
        let term_event_stream = term_subscription.event_stream();
        Subscription::run_with_id(self.term.id, term_event_stream).map(Message::Terminal)
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
            },
            Message::Update => self.term.input("cargo install --git https://github.com/Andrewkoro105/Archaeological_Assistant.git\nexit\n".to_string()),
            Message::Terminal(iced_term::Event::CommandReceived(_, cmd)) => {
                match self.term.update(cmd) {
                    iced_term::actions::Action::Shutdown => {
                        println!("{}", exec::Command::new(std::env::current_exe().unwrap()).exec());
                    }
                    _ => {},
                }
            },
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
