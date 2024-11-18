mod date;
mod input_info;

use crate::app::settings::insert_methods::InsertMethods;
use crate::app::{ArchaeologicalAssistant, Message};
use iced::widget::{button, column, container, row, text, text_input};
use iced::{alignment, Element, Length, Renderer, Theme};
use iced::advanced::graphics::text::cosmic_text::Align;

impl ArchaeologicalAssistant {
    pub fn view_main_menu(&self) -> impl Into<Element<Message, Theme, Renderer>> {
        column![
            row![self.view_date().into(), self.view_input_field().into(),],
            container(
                container(
                    column![
                        {
                            if self.settings.insert_methods_data.insert_methods
                                == InsertMethods::StartEnd
                            {
                                row![
                                    text("quantity:"),
                                    text_input::<Message, Theme, Renderer>("", &self.quantity.to_string())
                                        .on_input(Message::SetQuantity)
                                ]
                                .spacing(5)
                                .into()
                            } else {
                                Element::from(text("The field is only available in start/end mode"))
                            }
                        },
                        button("create")
                            .on_press(Message::Create)
                            .width(Length::Fill)
                    ]
                    .spacing(5)
                )
                .width(200)
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .align_y(alignment::Vertical::Bottom)
        ]
    }
}
