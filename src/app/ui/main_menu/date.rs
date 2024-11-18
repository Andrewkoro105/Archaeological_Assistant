use crate::app::{ArchaeologicalAssistant, Message};
use iced::widget::{
    column, container, Text,
};
use iced::{Element, Renderer, Theme};
use iced_aw::date_picker;

impl ArchaeologicalAssistant {
    pub fn view_date(&self) -> impl Into<Element<Message, Theme, Renderer>> {
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
        .spacing(12)
    }
}
