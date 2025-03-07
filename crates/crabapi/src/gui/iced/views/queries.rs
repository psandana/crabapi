use super::GUI;
use crate::gui::iced::{Message, TupleEvent, default_styles};
use iced::widget::{Button, Text, TextInput, column, container, row};
use iced::{Element, Length};

impl GUI {
    pub fn view_request_queries(&self) -> Element<Message> {
        container(self.view_request_queries_inner())
            .width(Length::Fill)
            .padding(default_styles::padding())
            .into()
    }

    pub fn view_request_queries_inner(&self) -> Element<Message> {
        let queries_title = Self::view_request_queries_title();

        let queries_column = self.view_request_queries_column();

        let query_add_button = Self::view_request_queries_add_button();

        column![queries_title, queries_column, query_add_button]
            .spacing(default_styles::spacing())
            .into()
    }

    fn view_request_queries_title() -> Element<'static, Message> {
        Text::new("Queries")
            .size(default_styles::input_size())
            .into()
    }

    fn view_request_queries_column(&self) -> Element<Message> {
        let mut queries_column = iced::widget::column![];

        for (i, query) in self.query_input.iter().enumerate() {
            let header_row = self.view_request_queries_row(i, query);
            queries_column = queries_column.push(header_row);
        }
        queries_column.spacing(default_styles::spacing()).into()
    }

    fn view_request_queries_row(
        &self,
        index: usize,
        header: &(String, String),
    ) -> Element<Message> {
        row![
            TextInput::new("Key", &header.0)
                .on_input(move |key| Message::QueryInputChanged(TupleEvent::KeyChanged(index, key)))
                .width(Length::FillPortion(1)),
            TextInput::new("Value", &header.1)
                .on_input(
                    move |value| Message::QueryInputChanged(TupleEvent::ValueChanged(index, value))
                )
                .width(Length::FillPortion(2)),
            Button::new(Text::new("X"))
                .on_press(Message::QueryInputChanged(TupleEvent::Remove(index)))
                .style(iced::widget::button::danger),
        ]
        .spacing(default_styles::spacing())
        .into()
    }

    fn view_request_queries_add_button() -> Element<'static, Message> {
        Button::new(Text::new("Add Query").size(default_styles::input_size()))
            .on_press(Message::QueryInputChanged(TupleEvent::Add))
            .into()
    }
}
