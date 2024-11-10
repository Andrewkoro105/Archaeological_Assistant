mod app;
mod settings;
mod theme;
mod data_base;
mod printer;

use app::ArchaeologicalAssistant;
use iced::Theme::{Dark, Dracula};
use iced_aw::iced_fonts;

pub fn main() {
    iced::application(
        "Archaeological assistant",
        ArchaeologicalAssistant::update,
        ArchaeologicalAssistant::view,
    )
    .font(iced_fonts::REQUIRED_FONT_BYTES)
    .theme(|app| app.theme())
    .run()
    .expect("Can't start the assistant");
}
