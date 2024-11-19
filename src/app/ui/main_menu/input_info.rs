use crate::app::data_base::table::Table;
use crate::app::data_base::DataBase;
use crate::app::settings::insert_methods::StartEnd;
use crate::app::settings::{
    insert_methods::{InsertMethods, InsertMethodsMessage},
    FieldType,
};
use crate::app::{ArchaeologicalAssistant, Message};
use iced::widget::{
    button, checkbox, column, combo_box, container, radio, row, text, text_input, Column,
};
use iced::{alignment, Element, Renderer, Theme};
use iced_aw::{TabLabel, Tabs};

impl ArchaeologicalAssistant {
    fn view_start_end_insert(&self) -> impl Into<Element<Message, Theme, Renderer>> {
        container(
            row![
                radio(
                    "End",
                    StartEnd::End,
                    Some(self.settings.insert_methods_data.start_end),
                    |data| Message::SetInsertMethodsData(InsertMethodsMessage::StartEnd(data))
                ),
                if self.is_can_start_insert {
                    radio(
                        "Start",
                        StartEnd::Start,
                        Some(self.settings.insert_methods_data.start_end),
                        |data| Message::SetInsertMethodsData(InsertMethodsMessage::StartEnd(data)),
                    )
                    .into()
                } else {
                    Element::from(row![
                        text("  |  "),
                        text("If the first index is equal to 1, the Start option is not available"),
                    ])
                }
            ]
            .spacing(5),
        )
        .padding(10)
    }

    fn view_input_insert(&self) -> impl Into<Element<Message, Theme, Renderer>> {
        container(
            column![
                text_input(
                    "Enter index",
                    &self
                        .settings
                        .insert_methods_data
                        .input_number_for_insert_methods_data
                        .input,
                )
                .on_input(|data| Message::SetInsertMethodsData(InsertMethodsMessage::Input(data))),
                if self.is_replace {
                    checkbox("are you sure you want to replace the data", self.on_replace)
                        .on_toggle(Message::OnReplace).into()
                } else {
                    Element::from(row![])
                }
            ]
            .spacing(5),
        )
        .padding(10)
    }

    fn view_auto_insert(&self) -> impl Into<Element<Message, Theme, Renderer>> {
        container(
            row![
                combo_box(
                    &self.state_auto_insert,
                    "Pick index",
                    Some(&self.settings.insert_methods_data.auto_insert),
                    |data| Message::SetInsertMethodsData(InsertMethodsMessage::AutoInsert(data))
                ),
                button("R").on_press(Message::RebootAutoInsertState)
            ]
            .spacing(5),
        )
        .padding(10)
    }

    pub fn view_input_field(&self) -> impl Into<Element<Message, Theme, Renderer>> {
        let input_fields = Column::with_children(
            (1..=self.settings.fields.len())
                .zip(&self.settings.fields)
                .map(|(i, field)| match field.field_type {
                    FieldType::Text => Element::from(
                        row![
                            text(field.name.clone())
                                .align_x(alignment::Horizontal::Right)
                                .width(100),
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
                text("Insets method")
                    .align_x(alignment::Horizontal::Right)
                    .width(100),
                Tabs::new(Message::SetInsertMethods)
                    .push(
                        InsertMethods::StartEnd,
                        TabLabel::Text("Start/End".to_string()),
                        self.view_start_end_insert(),
                    )
                    .push(
                        InsertMethods::Input,
                        TabLabel::Text("Input".to_string()),
                        self.view_input_insert(),
                    )
                    .push(
                        InsertMethods::AutoInsert,
                        TabLabel::Text("Auto insert".to_string()),
                        self.view_auto_insert(),
                    )
                    .set_active_tab(&self.settings.insert_methods_data.insert_methods),
            ]
            .spacing(5),
            input_fields
        ]
    }
}
