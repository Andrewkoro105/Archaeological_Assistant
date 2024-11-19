use crate::app::{ArchaeologicalAssistant, Message};
use iced::{
    widget::{button, column, container},
    Element, Length, Renderer, Theme,
};
use iced_term::TerminalView;

impl ArchaeologicalAssistant {
    pub fn view_update_menu(&self) -> impl Into<Element<Message, Theme, Renderer>> {
        column![
            container(button("Update").on_press(Message::Update)).padding(10),
            container(TerminalView::show(&self.term).map(Message::Terminal))
                .width(Length::Fill)
                .height(Length::Fill)
        ]
    }
}
