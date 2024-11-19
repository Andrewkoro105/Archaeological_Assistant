mod date;
mod input_info;

use crate::app::settings::insert_methods::InsertMethods;
use crate::app::{ArchaeologicalAssistant, Message};
use iced::widget::{button, column, container, row, text, text_input};
use iced::{alignment, Color, Element, Length, Renderer, Theme};

impl ArchaeologicalAssistant {
    pub fn view_main_menu(&self) -> impl Into<Element<Message, Theme, Renderer>> {
        column![
            row![
                self.view_date().into(),
                column![self.view_input_field().into(),]
            ],
            container(
                row![
                    if self.is_replace {
                        text("!!! This record already exist")
                            .color(Color::new(1., 1., 0., 1.))
                            .height(Length::Fill)
                            .align_y(alignment::Vertical::Bottom)
                            .into()
                    } else {
                        Element::from(row![])
                    },
                    container(
                        column![
                            {
                                if self.settings.insert_methods_data.insert_methods
                                    == InsertMethods::StartEnd
                                {
                                    row![
                                        text("quantity:"),
                                        text_input::<Message, Theme, Renderer>(
                                            "",
                                            &self.quantity.to_string()
                                        )
                                        .on_input(Message::SetQuantity)
                                    ]
                                    .spacing(5)
                                    .into()
                                } else {
                                    Element::from(text(
                                        "The field is only available in start/end mode",
                                    ))
                                }
                            },
                            button(if self.is_replace {
                                if self.on_replace {
                                    "Replace"
                                } else {
                                    "Not replace"
                                }
                            } else {
                                "create"
                            })
                            .on_press(Message::Create)
                            .width(Length::Fill)
                        ]
                        .spacing(5)
                    )
                    .height(Length::Fill)
                    .align_y(alignment::Vertical::Bottom)
                    .width(200)
                ]
                .spacing(5)
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .align_y(alignment::Vertical::Bottom)
        ]
    }
}
