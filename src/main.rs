mod app;

use app::ArchaeologicalAssistant;
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
