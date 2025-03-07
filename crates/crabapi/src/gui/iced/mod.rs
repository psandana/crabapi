// internal mods
mod default_styles;

use http::{HeaderMap, HeaderName};
// dependencies
use crate::core::requests;
use iced;
use iced::widget::text_editor::{Action, Content};
use iced::widget::{Button, Row, Text, TextInput, scrollable, text_editor};
use iced::widget::{button, column, container, pick_list, row};
use iced::{Alignment, Center, Element, Length, Task};
use iced_highlighter::Highlighter;
use reqwest::{Body, Client};
// internal dependencies
use crate::core::requests::{Method, constants, send_requests, validators};

pub fn init() {
    iced::run(GUI::title, GUI::update, GUI::view).unwrap()
}

#[derive(Debug, Clone)]
enum Message {
    MethodChanged(Method),
    UrlInputChanged(String),
    HeaderInputChanged(TupleEvent),
    QueryInputChanged(TupleEvent),
    SendRequest,
    ResponseBodyChanged(String),
    ResponseBodyText(Action),
}

#[derive(Debug, Clone)]
enum TupleEvent {
    KeyChanged(usize, String),
    ValueChanged(usize, String),
    Remove(usize),
    Add,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
struct GUI {
    client: Client,
    methods: &'static [Method],
    method_selected: Option<Method>,
    url_input: String,
    url_input_valid: bool,
    query_input: Vec<(String, String)>,
    header_input: Vec<(String, String)>,
    response_body: Content,
}

impl GUI {
    fn new() -> Self {
        Self {
            client: Client::new(),
            methods: &constants::METHODS,
            method_selected: Some(Method::GET),
            url_input: String::new(),
            url_input_valid: false,
            query_input: vec![(String::new(), String::new())],
            header_input: vec![(String::new(), String::new())],
            response_body: Content::with_text("Response body will go here..."),
        }
    }

    fn title(&self) -> String {
        crate::core::app::constants::APP_NAME.to_string()
    }

    fn update(&mut self, event: Message) -> Task<Message> {
        match event {
            Message::MethodChanged(method) => {
                self.method_selected = Some(method);
                Task::none()
            }
            Message::UrlInputChanged(url) => {
                self.url_input = url;
                self.url_input_valid = validators::is_valid_url(&self.url_input);
                Task::none()
            }
            Message::HeaderInputChanged(header_message) => {
                Self::update_tuple(&mut self.header_input, header_message)
            }
            Message::QueryInputChanged(query_message) => {
                Self::update_tuple(&mut self.query_input, query_message)
            }
            Message::SendRequest => {
                self.url_input_valid = validators::is_valid_url(&self.url_input);
                if !self.url_input_valid {
                    return Task::none();
                }

                let mut headers = HeaderMap::new();
                for (key, value) in self.header_input.iter() {
                    if key.is_empty() {
                        continue;
                    }

                    headers.insert(
                        HeaderName::from_lowercase(key.to_lowercase().as_ref()).unwrap(),
                        value.parse().unwrap(),
                    );
                }

                let request = requests::build_request(
                    &self.client,
                    self.url_input.parse().unwrap(),
                    self.query_input.clone(),
                    self.method_selected.clone().unwrap(),
                    headers,
                    Body::from(String::new()),
                );

                let handles = send_requests(vec![request]);
                let handle = handles.into_iter().nth(0).unwrap();
                Task::perform(
                    async move { handle.await.unwrap().unwrap().text().await },
                    |result| match result {
                        Ok(response) => Message::ResponseBodyChanged(response),
                        Err(error) => Message::ResponseBodyChanged(error.to_string()),
                    },
                )
            }
            Message::ResponseBodyChanged(response) => {
                self.response_body = Content::with_text(&response);
                Task::none()
            }
            Message::ResponseBodyText(action) => {
                match action {
                    Action::Edit(_text) => {}
                    _ => {
                        self.response_body.perform(action);
                    }
                }

                Task::none()
            }
        }
    }

    fn update_tuple(tuple_vec: &mut Vec<(String, String)>, message: TupleEvent) -> Task<Message> {
        match message {
            TupleEvent::KeyChanged(index, key) => {
                if let Some(tuple) = tuple_vec.get_mut(index) {
                    tuple.0 = key;
                }
                Task::none()
            }
            TupleEvent::ValueChanged(index, value) => {
                if let Some(tuple) = tuple_vec.get_mut(index) {
                    tuple.1 = value;
                }
                Task::none()
            }
            TupleEvent::Add => {
                tuple_vec.push((String::new(), String::new()));
                Task::none()
            }
            TupleEvent::Remove(index) => {
                tuple_vec.remove(index);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // ROW: Method, URI, Send Button
        let request_row = self.view_request();

        // ROW: Headers
        let headers_column = self.view_request_headers();

        // ROW: Queries
        let queries_column = self.view_request_queries();

        // ROW: Response
        let response_row = self.view_response();

        column![
            request_row,
            container(headers_column)
                .width(Length::Fill)
                .padding(default_styles::padding()),
            container(queries_column)
                .width(Length::Fill)
                .padding(default_styles::padding()),
            container(response_row)
                .align_x(Center)
                .width(Length::Fill)
                .padding(default_styles::padding()),
        ]
        .into()
    }

    fn view_request(&self) -> Element<Message> {
        let url_input = self.view_request_url_input();

        let method_input = self.view_request_method_input();

        let send_button = Self::view_request_send_button();

        let request_row = Self::view_request_row_setup(row![method_input, url_input, send_button]);

        request_row.into()
    }

    // VIEW REQUEST - GENERAL

    fn view_request_url_input(&self) -> Element<Message> {
        let url_input_icon = Self::view_request_url_input_icon(self.url_input_valid);
        let url_input = TextInput::new("Enter URI", &self.url_input)
            .on_input(Message::UrlInputChanged)
            .size(default_styles::input_size())
            .icon(url_input_icon)
            .width(Length::Fill);

        url_input.into()
    }

    fn view_request_url_input_icon(valid: bool) -> iced::widget::text_input::Icon<iced::Font> {
        iced::widget::text_input::Icon {
            font: iced::Font::default(),
            code_point: if valid { '✅' } else { '❌' },
            size: Some(default_styles::input_size()),
            spacing: 0.0,
            side: iced::widget::text_input::Side::Right,
        }
    }

    fn view_request_method_input(&self) -> Element<Message> {
        pick_list(
            self.methods,
            self.method_selected.clone(),
            Message::MethodChanged,
        )
        .placeholder("Method")
        .width(Length::Shrink)
        .text_size(default_styles::input_size())
        .into()
    }

    fn view_request_row_setup(request_row: Row<'_, Message>) -> Row<'_, Message> {
        request_row
            .spacing(default_styles::spacing())
            .padding(default_styles::padding())
            .align_y(Alignment::Center)
    }

    fn view_request_send_button() -> Element<'static, Message> {
        Button::new(Text::new("Send").size(default_styles::input_size()))
            .on_press(Message::SendRequest)
            .into()
    }

    // VIEW REQUEST - HEADERS

    fn view_request_headers(&self) -> Element<Message> {
        let headers_title = Self::view_request_headers_title();

        let headers_column = self.view_request_headers_column();

        let header_add_button = Self::view_request_headers_add_button();

        column![headers_title, headers_column, header_add_button]
            .spacing(default_styles::spacing())
            .into()
    }

    fn view_request_headers_title() -> Element<'static, Message> {
        Text::new("Headers").size(16).into()
    }

    fn view_request_headers_column(&self) -> Element<Message> {
        let mut headers_column = column![];

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
            TextInput::new("Value", &header.1) // TODO: Change unwrap
                .on_input(
                    move |value| Message::HeaderInputChanged(TupleEvent::ValueChanged(
                        index, value
                    ))
                )
                .width(Length::FillPortion(2)),
            Button::new(Text::new("X"))
                .on_press(Message::HeaderInputChanged(TupleEvent::Remove(index)))
                .style(button::danger),
        ]
        .spacing(default_styles::spacing())
        .into()
    }

    fn view_request_headers_add_button() -> Element<'static, Message> {
        Button::new(Text::new("Add Header").size(default_styles::input_size()))
            .on_press(Message::HeaderInputChanged(TupleEvent::Add))
            .into()
    }

    // VIEW REQUEST QUERIES
    fn view_request_queries(&self) -> Element<Message> {
        let queries_title = Self::view_request_queries_title();

        let queries_column = self.view_request_queries_column();

        let query_add_button = Self::view_request_queries_add_button();

        column![queries_title, queries_column, query_add_button]
            .spacing(default_styles::spacing())
            .into()
    }

    fn view_request_queries_title() -> Element<'static, Message> {
        Text::new("Queries").size(16).into()
    }

    fn view_request_queries_column(&self) -> Element<Message> {
        let mut queries_column = column![];

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
            TextInput::new("Value", &header.1) // TODO: Change unwrap
                .on_input(
                    move |value| Message::QueryInputChanged(TupleEvent::ValueChanged(index, value))
                )
                .width(Length::FillPortion(2)),
            Button::new(Text::new("X"))
                .on_press(Message::QueryInputChanged(TupleEvent::Remove(index)))
                .style(button::danger),
        ]
        .spacing(default_styles::spacing())
        .into()
    }

    fn view_request_queries_add_button() -> Element<'static, Message> {
        Button::new(Text::new("Add Query").size(default_styles::input_size()))
            .on_press(Message::QueryInputChanged(TupleEvent::Add))
            .into()
    }

    fn view_response(&self) -> Element<'_, Message> {
        let label = Text::new("Response:").size(default_styles::input_size());
        let body = text_editor(&self.response_body)
            .on_action(Message::ResponseBodyText)
            .highlight_with::<Highlighter>(
                iced_highlighter::Settings {
                    theme: iced_highlighter::Theme::SolarizedDark,
                    token: "html".to_string(),
                },
                |highlight, _theme| highlight.to_format(),
            );
        column![label, scrollable(body)].into()
    }
}

impl Default for GUI {
    fn default() -> Self {
        GUI::new()
    }
}
