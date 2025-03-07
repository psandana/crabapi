// internal mods
mod default_styles;

use http::{HeaderMap, HeaderName};
use std::collections::HashMap;
// dependencies
use crate::core::requests;
use iced;
use iced::widget::text_editor::{Action, Content};
use iced::widget::{Button, Row, Text, TextInput, scrollable, text_editor};
use iced::widget::{button, column, container, pick_list, radio, row};
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
    HeaderKeyChanged(usize, String),
    HeaderValueChanged(usize, String),
    RemoveHeader(usize),
    AddHeader,
    SendRequest,
    ResponseBodyChanged(String),
    ResponseBodyText(Action),
    BodyTypeChanged(BodyType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BodyType {
    File,
    Text,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
struct GUI {
    client: Client,
    methods: &'static [Method],
    method_selected: Option<Method>,
    url_input: String,
    url_input_valid: bool,
    header_input: Vec<(String, String)>,
    response_body: Content,
    // body_input: String,
    body_type_select: Option<BodyType>,
}

impl GUI {
    fn new() -> Self {
        Self {
            client: Client::new(),
            methods: &constants::METHODS,
            method_selected: Some(Method::GET),
            url_input: String::new(),
            url_input_valid: false,
            header_input: vec![(String::new(), String::new())],
            response_body: Content::with_text("Response body will go here..."),
            body_type_select: Some(BodyType::Text),
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
            Message::HeaderKeyChanged(index, key) => {
                if let Some(header) = self.header_input.get_mut(index) {
                    header.0 = key;
                }
                Task::none()
            }
            Message::HeaderValueChanged(index, value) => {
                if let Some(header) = self.header_input.get_mut(index) {
                    header.1 = value;
                }
                Task::none()
            }
            Message::AddHeader => {
                self.header_input.push((String::new(), String::new()));
                Task::none()
            }
            Message::RemoveHeader(index) => {
                self.header_input.remove(index);
                Task::none()
            }
            Message::SendRequest => {
                self.url_input_valid = validators::is_valid_url(&self.url_input);

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
                    HashMap::new(), // TODO: query
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
            Message::BodyTypeChanged(body_type) => {
                println!("Body Type Changed: {:?}", body_type);
                self.body_type_select = Some(body_type);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // ROW: Method, URI, Send Button
        let request_row = self.view_request();

        // ROW: Headers
        let headers_row = self.view_request_headers();

        // ROW: Body
        let body_row = self.view_request_body();

        // ROW: Response
        let response_row = self.view_response();

        column![request_row, headers_row, body_row, response_row].into()
    }

    fn view_request(&self) -> Element<Message> {
        let url_input = self.view_request_url_input();

        let method_input = self.view_request_method_input();

        let send_button = Self::view_request_send_button();

        let request_row = Self::view_request_row_setup(row![method_input, url_input, send_button]);

        let title_row = Self::view_request_row_setup(row![Self::view_request_title()]);

        column![title_row, request_row].into()
    }

    // VIEW REQUEST - GENERAL

    fn view_request_title() -> Element<'static, Message> {
        Text::new("Request")
            .size(default_styles::input_size())
            .into()
    }

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
        container(self.view_request_headers_inner())
            .width(Length::Fill)
            .padding(default_styles::padding())
            .into()
    }

    fn view_request_headers_inner(&self) -> Element<Message> {
        let headers_title = Self::view_request_headers_title();

        let headers_column = self.view_request_headers_column();

        let header_add_button = Self::view_request_headers_add_button();

        column![headers_title, headers_column, header_add_button]
            .spacing(default_styles::spacing())
            .into()
    }

    fn view_request_headers_title() -> Element<'static, Message> {
        Text::new("Headers")
            .size(default_styles::input_size())
            .into()
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
                .on_input(move |key| Message::HeaderKeyChanged(index, key))
                .width(Length::FillPortion(1)),
            TextInput::new("Value", &header.1) // TODO: Change unwrap
                .on_input(move |value| Message::HeaderValueChanged(index, value))
                .width(Length::FillPortion(2)),
            Button::new(Text::new("X"))
                .on_press(Message::RemoveHeader(index))
                .style(button::danger)
        ]
        .spacing(default_styles::spacing())
        .into()
    }

    fn view_request_headers_add_button() -> Element<'static, Message> {
        Button::new(Text::new("Add Header").size(default_styles::input_size()))
            .on_press(Message::AddHeader)
            .into()
    }

    // VIEW REQUEST - BODY

    fn view_request_body(&self) -> Element<Message> {
        container(self.view_request_body_inner())
            .width(Length::Fill)
            .padding(default_styles::padding())
            .into()
    }

    fn view_request_body_inner(&self) -> Element<Message> {
      let body_title = Self::view_request_body_title();

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

      column!(
          body_title,
          row![text, file].spacing(default_styles::spacing())
      )
      .into()
    }

    fn view_request_body_title() -> Element<'static, Message> {
        Text::new("Body").size(default_styles::input_size()).into()
    }

    // VIEW RESPONSE

    fn view_response(&self) -> Element<'_, Message> {
        container(self.view_response_inner())
            .align_x(Center)
            .width(Length::Fill)
            .padding(default_styles::padding())
            .into()
    }

    fn view_response_inner(&self) -> Element<'_, Message> {
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
