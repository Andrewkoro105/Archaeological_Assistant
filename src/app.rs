use crate::data_base::DataBase;
use crate::settings::{Axis, FieldType, Settings};
use iced::widget::{button, column, container, row, text, text_input, Column, Text};
use iced::{alignment, Element, Length, Renderer, Theme};
use iced_aw::{date_picker, selection_list, TabLabel, Tabs};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Message {
    None,
    Create,
    SetData(String, usize),
    SetQuantity(String),
    SetMenu(MenuStatus),
    SelectDb(Vec<(&'static str, &'static [&'static str])>),
    SelectFont(Vec<(&'static str, &'static [&'static str])>),
    SetPathToDb(String),
    SetPathToFont(String),
    SetSize(String, Axis),
    SetSizeImage(String, Axis),
    SetPositionImage(String, Axis),
    SetHeightText(String),
    SetTextSize(String),
    SetTheme(usize, &'static str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum MenuStatus {
    Main,
    Settings,
}

pub struct ArchaeologicalAssistant {
    menu_status: MenuStatus,
    data: Vec<String>,
    quantity: usize,
    settings: Settings,
}

impl Default for ArchaeologicalAssistant {
    fn default() -> Self {
        let settings = Settings::load();
        Self {
            menu_status: MenuStatus::Main,
            data: {
                [
                    vec![date_picker::Date::today().to_string()],
                    DataBase::get_end_data(&settings.path_to_db, settings.fields.len(), 3),
                ]
                .concat()
            },
            quantity: 0,
            settings,
        }
    }
}

impl ArchaeologicalAssistant {
    pub fn theme(&self) -> Theme {
        self.settings.theme.to_iced_theme()
    }

    pub fn update(&mut self, message: Message) {
        match self.settings.update(message) {
            Message::None => {}
            Message::SetData(str, id) => self.data[id] = str,
            Message::SetQuantity(quantity) => {
                self.quantity = quantity.parse().unwrap_or(self.quantity)
            }
            Message::SetMenu(menu_status) => {
                if self.menu_status == MenuStatus::Settings && self.menu_status != menu_status {
                    self.settings.save()
                }
                self.menu_status = menu_status
            }
            Message::Create => DataBase::create_record(
                &self.settings.path_to_db,
                &self.settings.print_settings,
                self.quantity,
                self.data.clone(),
            ),
            _ => panic!("Message not found"),
        }
    }

    fn view_main_menu(&self) -> impl Into<Element<Message, Theme, Renderer>> {
        let input_fields = Column::with_children(
            (1..=self.settings.fields.len())
                .zip(&self.settings.fields)
                .map(|(i, field)| match field.field_type {
                    FieldType::Text => Element::from(
                        row![
                            container(text(field.name.clone())).align_right(100),
                            text_input(
                                &*("Enter ".to_string() + &field.name.to_lowercase()),
                                &self.data[i]
                            )
                            .on_input(move |str| Message::SetData(str, i))
                        ]
                        .spacing(5),
                    ),
                })
                .collect::<Vec<_>>(),
        )
        .spacing(12);

        column![
            row![
                column![
                    container(Text::new(&self.data[0])).center_x(300),
                    container(
                        date_picker(
                            true,
                            date_picker::Date::today(),
                            Text::new(""),
                            Message::None,
                            |date| Message::SetData(date.to_string(), 0),
                        )
                        .font_size(12)
                    )
                    .padding(150)
                ]
                .spacing(12),
                input_fields
            ],
            container(
                container(column![
                    row![
                        text("quantity:"),
                        text_input("", &self.quantity.to_string())
                        .on_input(Message::SetQuantity)
                    ]
                    .spacing(5),
                    button("create")
                        .on_press(Message::Create)
                        .width(Length::Fill)
                ]
                .spacing(5))
                .width(200)
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .align_y(alignment::Vertical::Bottom)
        ]
    }

    fn create_param<'elem>(
        placeholder: &'elem str,
        input: Element<'elem, Message, Theme, Renderer>,
    ) -> Element<'elem, Message, Theme, Renderer> {
        Element::from(row![container(text(placeholder)).align_right(200), input].spacing(5))
    }

    fn create_1_param<'elem, F>(
        placeholder: &'elem str,
        value: impl ToString,
        message: F,
    ) -> Element<'elem, Message, Theme, Renderer>
    where
        F: Fn(String) -> Message + 'elem,
    {
        Self::create_param(
            placeholder,
            text_input("", &value.to_string()).on_input(message).into(),
        )
    }

    fn create_2_param<'elem, F>(
        placeholder: &'elem str,
        value: (impl ToString, impl ToString),
        message: F,
    ) -> Element<'elem, Message, Theme, Renderer>
    where
        F: Fn(String, Axis) -> Message + Clone + 'elem,
    {
        let message1 = message.clone();
        Self::create_param(
            placeholder,
            row![
                text_input("", &value.0.to_string()).on_input(move |str| message1(str, Axis::X)),
                text_input("", &value.1.to_string()).on_input(move |str| message(str, Axis::Y))
            ]
            .spacing(12)
            .into(),
        )
    }

    fn create_path_param<'elem, TextInputMessage, ButtonMessage>(
        placeholder: &'elem str,
        value: impl ToString,
        text_input_message: TextInputMessage,
        button_message: ButtonMessage,
        file_types: Vec<(&'static str, &'static [&'static str])>,
    ) -> Element<'elem, Message, Theme, Renderer>
    where
        TextInputMessage: Fn(String) -> Message + Clone + 'elem,
        ButtonMessage: Fn(Vec<(&'static str, &'static [&'static str])>) -> Message + Clone + 'elem,
    {
        Self::create_param(
            placeholder,
            row![
                Element::from(text_input("", &value.to_string()).on_input(text_input_message)),
                Element::from(button("select file").on_press(button_message(file_types))),
            ]
            .spacing(12)
            .into(),
        )
    }

    fn view_settings_menu(&self) -> impl Into<Element<Message, Theme, Renderer>> {
        let menu_settings = vec![
            text("Base").into(),
            Self::create_path_param(
                "path to db",
                self.settings
                    .path_to_db
                    .to_str()
                    .expect("Can't convert path to db in str"),
                Message::SetPathToDb,
                Message::SelectDb,
                vec![
                    (
                        "excel files",
                        &["xls", "xlsx", "xlsm", "xlsb", "xla", "xlam"],
                    ),
                    ("opendocument spreadsheets", &["ods"]),
                ],
            ),
            text("Print Settings").into(),
            Self::create_path_param(
                "path to font",
                self.settings
                    .print_settings
                    .font
                    .to_str()
                    .expect("Can't convert path to db in str"),
                Message::SetPathToFont,
                Message::SelectFont,
                vec![("font files", &["ttf"])],
            ),
            Self::create_2_param("size", self.settings.print_settings.size, Message::SetSize),
            Self::create_2_param(
                "size image",
                self.settings.print_settings.size_image,
                Message::SetSizeImage,
            ),
            Self::create_2_param(
                "position image",
                self.settings.print_settings.pos_image,
                Message::SetPositionImage,
            ),
            Self::create_1_param(
                "text size",
                self.settings.print_settings.text_size,
                Message::SetTextSize,
            ),
            Self::create_1_param(
                "Y position of text",
                self.settings.print_settings.text_size,
                Message::SetTextSize,
            ),
            text("Application").into(),
            Self::create_param(
                "Theme",
                selection_list(
                    &[
                        "Light",
                        "Dark",
                        "Dracula",
                        "Nord",
                        "SolarizedLight",
                        "SolarizedDark",
                        "GruvboxLight",
                        "GruvboxDark",
                        "CatppuccinLatte",
                        "CatppuccinFrappe",
                        "CatppuccinMacchiato",
                        "CatppuccinMocha",
                        "TokyoNight",
                        "TokyoNightStorm",
                        "TokyoNightLight",
                        "KanagawaWave",
                        "KanagawaDragon",
                        "KanagawaLotus",
                        "Moonfly",
                        "Nightfly",
                        "Oxocarbon",
                        "Ferra",
                    ],
                    Message::SetTheme,
                )
                .height(100)
                .into(),
            ),
        ];

        Column::with_children(menu_settings).spacing(12)
    }

    pub fn view(&self) -> Element<Message> {
        container(
            Tabs::new(Message::SetMenu)
                .push(
                    MenuStatus::Main,
                    TabLabel::Text("Main".to_string()),
                    container(self.view_main_menu()).padding(10),
                )
                .push(
                    MenuStatus::Settings,
                    TabLabel::Text("Settings".to_string()),
                    container(self.view_settings_menu()).padding(10),
                )
                .set_active_tab(&self.menu_status),
        )
        .padding(20)
        .into()
    }
}