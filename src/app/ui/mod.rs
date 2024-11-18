mod main_menu;
mod settings;

use crate::app::settings::{Axis, MessageSettings};
use crate::app::{ArchaeologicalAssistant, MenuStatus, Message};
use iced::widget::{button, combo_box, container, row, text, text_input, Column};
use iced::{Element, Renderer, Theme};
use iced_aw::{TabLabel, Tabs};

impl ArchaeologicalAssistant {
    fn create_param<'elem>(
        placeholder: &'elem str,
        input: Element<'elem, Message, Theme, Renderer>,
    ) -> Element<'elem, Message, Theme, Renderer> {
        Element::from(row![container(text(placeholder)).align_right(200), input].spacing(5))
    }

    fn create_1_param<'elem, F>(
        placeholder: &'elem str,
        value: &impl ToString,
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
        value: &(impl ToString, impl ToString),
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
