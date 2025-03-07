use super::GUI;
use super::super::BodyType;
use crate::gui::iced::{Message, default_styles};
use iced::widget::{Button, Row, Space, Text, column, container, radio, row, text_editor};
use iced::{Center, Element, Length};

impl GUI {
    pub fn view_request_body(&self) -> Element<Message> {
        container(self.view_request_body_inner())
            .width(Length::Fill)
            .padding(default_styles::padding())
            .into()
    }

    fn view_request_body_inner(&self) -> Element<Message> {
        let body_title = Self::view_request_body_title();

        let radio_buttons = self.view_request_body_radio_buttons();

        let content = self.view_request_body_content();

        column!(body_title, radio_buttons, content,)
            .spacing(default_styles::spacing())
            .into()
    }

    fn view_request_body_title() -> Element<'static, Message> {
        Text::new("Body").size(default_styles::input_size()).into()
    }

    fn view_request_body_radio_buttons(&self) -> Row<Message> {
        let empty = radio(
            "Empty",
            BodyType::Empty,
            self.body_type_select,
            Message::BodyTypeChanged,
        );

        let text = radio(
            "Text",
            BodyType::Text,
            self.body_type_select,
            Message::BodyTypeChanged,
        );
        let file = radio(
            "File",
            BodyType::File,
            self.body_type_select,
            Message::BodyTypeChanged,
        );

        row![empty, text, file].spacing(default_styles::spacing())
    }

    fn view_request_body_content(&self) -> Row<Message> {
        let content = match self.body_type_select {
            Some(BodyType::Empty) => row![],
            Some(BodyType::File) => self.view_request_body_file(),
            Some(BodyType::Text) => self.view_request_body_text(),
            None => row![],
        };

        content
    }

    fn view_request_body_text(&self) -> Row<Message> {
        row![
            text_editor(&self.body_content)
                .on_action(Message::BodyContentChanged)
                .placeholder("Introduce body here...")
                .size(default_styles::input_size())
        ]
    }

    fn view_request_body_file(&self) -> Row<Message> {
        let file_name_string = format!(
            "File: {}",
            self.body_file_path
                .as_ref()
                .map(|path| path.to_string_lossy().to_string())
                .unwrap_or_else(|| "No file selected".to_string())
        );

        row![
            Self::view_request_body_text_button(),
            Space::new(default_styles::input_size(), default_styles::input_size()),
            Text::new(file_name_string).size(default_styles::input_size())
        ]
        .align_y(Center)
    }

    fn view_request_body_text_button() -> Element<'static, Message> {
        Button::new(Text::new("Select File").size(default_styles::input_size()))
            .on_press(Message::BodyContentOpenFile)
            .into()
    }
}
