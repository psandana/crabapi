use super::GUI;
use crate::gui::iced::{Message, TupleEvent, default_styles};
use iced::widget::{Button, Text, TextInput, container, row};
use iced::{Element, Length};

impl GUI {
    pub fn view_request_headers(&self) -> Element<Message> {
        container(self.view_request_headers_inner())
            .width(Length::Fill)
            .padding(default_styles::padding())
            .into()
    }

    fn view_request_headers_inner(&self) -> Element<Message> {
        let headers_title = Self::view_request_headers_title();

        let headers_column = self.view_request_headers_column();

        let header_add_button = Self::view_request_headers_add_button();

        iced::widget::column![headers_title, headers_column, header_add_button]
            .spacing(default_styles::spacing())
            .into()
    }

    fn view_request_headers_title() -> Element<'static, Message> {
        Text::new("Headers")
            .size(default_styles::input_size())
            .into()
    }

    fn view_request_headers_column(&self) -> Element<Message> {
        let mut headers_column = iced::widget::column![];

        for (i, header) in self.header_input.iter().enumerate() {
            let header_row = self.view_request_headers_column_row(i, header);
            headers_column = headers_column.push(header_row);
        }
        headers_column.spacing(default_styles::spacing()).into()
    }

    fn view_request_headers_column_row(
        &self,
        index: usize,
        header: &(String, String),
    ) -> Element<Message> {
        row![
            TextInput::new("Key", &header.0)
                .on_input(
                    move |key| Message::HeaderInputChanged(TupleEvent::KeyChanged(index, key))
                )
                .width(Length::FillPortion(1)),
            TextInput::new("Value", &header.1)
                .on_input(
                    move |value| Message::HeaderInputChanged(TupleEvent::ValueChanged(
                        index, value
                    ))
                )
                .width(Length::FillPortion(2)),
            Button::new(Text::new("X"))
                .on_press(Message::HeaderInputChanged(TupleEvent::Remove(index)))
                .style(iced::widget::button::danger),
        ]
        .spacing(default_styles::spacing())
        .into()
    }

    fn view_request_headers_add_button() -> Element<'static, Message> {
        Button::new(Text::new("Add Header").size(default_styles::input_size()))
            .on_press(Message::HeaderInputChanged(TupleEvent::Add))
            .into()
    }
}
