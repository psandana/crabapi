// internal mods
mod default_styles;

// dependencies
use iced;
use iced::widget::{Button, Row, Text, TextInput};
use iced::widget::{button, column, container, pick_list, radio, row};
use iced::{Alignment, Element, Length};

// internal dependencies
use crate::core::requests::{Method, constants, validators};

pub fn init() {
    iced::run(GUI::title, GUI::update, GUI::view).unwrap()
}

#[derive(Debug, Clone)]
enum Message {
    MethodChanged(Method),
    UrlInputChanged(String),
    SendRequest,
    HeaderKeyChanged(usize, String),
    HeaderValueChanged(usize, String),
    #[allow(dead_code)] // TODO: Remove this out-out warning
    RemoveHeader(usize),
    AddHeader,
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
    methods: &'static [Method],
    method_selected: Option<Method>,
    url_input: String,
    url_input_valid: bool,
    header_input: Vec<(String, String)>,
    // body_input: String,
    body_type_select: Option<BodyType>,
}

impl GUI {
    fn new() -> Self {
        Self {
            methods: &constants::METHODS,
            method_selected: Some(Method::GET),
            url_input: String::new(),
            url_input_valid: false,
            header_input: vec![(String::new(), String::new())],
            body_type_select: Some(BodyType::Text),
        }
    }

    fn title(&self) -> String {
        crate::core::app::constants::APP_NAME.to_string()
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::MethodChanged(method) => {
                self.method_selected = Some(method);
            }
            Message::UrlInputChanged(url) => {
                self.url_input = url;
                self.url_input_valid = validators::is_valid_url(&self.url_input);
            }
            Message::SendRequest => {
                // TODO
            }
            Message::HeaderKeyChanged(index, key) => {
                if let Some(header) = self.header_input.get_mut(index) {
                    header.0 = key;
                }
            }
            Message::HeaderValueChanged(index, value) => {
                if let Some(header) = self.header_input.get_mut(index) {
                    header.1 = value;
                }
            }
            Message::AddHeader => {
                self.header_input.push((String::new(), String::new()));
            }
            Message::BodyTypeChanged(body_type) => {
                println!("Body Type Changed: {:?}", body_type);
                self.body_type_select = Some(body_type);
            }
            _ => {} // TODO: REmove this. Unnecessary if all implemented and enum is non-exhaustive
        }
    }

    fn view(&self) -> Element<Message> {
        // ROW: Method, URI, Send Button
        let request_row = self.view_request();

        // ROW: Headers
        let headers_row = self.view_request_headers();

        // ROW: Body
        let body_row = self.view_request_body();

        column![request_row, headers_row, body_row].into()
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

        column!(body_title, row![text, file].spacing(default_styles::spacing())).into()
    }

    fn view_request_body_title() -> Element<'static, Message> {
        Text::new("Body")
            .size(default_styles::input_size())
            .into()
    }
}

impl Default for GUI {
    fn default() -> Self {
        GUI::new()
    }
}
