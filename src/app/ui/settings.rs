use iced::{Element, Renderer, Theme};
use iced::widget::{combo_box, text, Column};
use crate::app::{ArchaeologicalAssistant, Message};
use crate::app::settings::MessageSettings;

impl ArchaeologicalAssistant {
    pub fn view_settings_menu(&self) -> impl Into<Element<Message, Theme, Renderer>> {
        let menu_settings = vec![
            text("Base").into(),
            Self::create_path_param(
                "path to db",
                self.settings
                    .path_to_db
                    .to_str()
                    .expect("Can't convert path to db in str"),
                |path| Message::SetSettings(MessageSettings::SetPathToDb(path)),
                |path| Message::SetSettings(MessageSettings::SelectDb(path)),
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
                |path| Message::SetSettings(MessageSettings::SetPathToFont(path)),
                |path| Message::SetSettings(MessageSettings::SelectFont(path)),
                vec![("font files", &["ttf"])],
            ),
            Self::create_2_param(
                "size",
                &self.settings.print_settings.input_number.size,
                |size, axis| Message::SetSettings(MessageSettings::SetSize(size, axis)),
            ),
            Self::create_2_param(
                "size image",
                &self.settings.print_settings.input_number.size_image,
                |size, axis| Message::SetSettings(MessageSettings::SetSizeImage(size, axis)),
            ),
            Self::create_2_param(
                "position image",
                &self.settings.print_settings.input_number.pos_image,
                |position, axis| Message::SetSettings(MessageSettings::SetPositionImage(position, axis)),
            ),
            Self::create_1_param(
                "text size",
                &self.settings.print_settings.input_number.text_size,
                |size| Message::SetSettings(MessageSettings::SetTextSize(size)),
            ),
            Self::create_1_param(
                "Y position of text",
                &self.settings.print_settings.input_number.height_text,
                |height| Message::SetSettings(MessageSettings::SetHeightText(height)),
            ),
            text("Application").into(),
            Self::create_param(
                "Theme",
                combo_box(
                    &self.state_themes,
                    "Select theme",
                    Some(&self.settings.theme),
                    |theme| Message::SetSettings(MessageSettings::SetTheme(theme)),
                )
                    .into(),
            ),
        ];

        Column::with_children(menu_settings).spacing(12)
    }
}