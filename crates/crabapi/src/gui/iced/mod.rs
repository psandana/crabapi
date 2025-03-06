use iced;
use iced::widget::{Button, Text, TextInput, column, row};
use iced::widget::{button, combo_box, container};
use iced::{Element, Length};

use crate::core::requests::{Method, constants};

pub fn init() {
    iced::run(GUI::title, GUI::update, GUI::view).unwrap()
}

#[derive(Debug, Clone)]
enum Message {
    MethodChanged(Method),
    UrlInputChanged(String),
    // HeaderInputChanged(String),
    // BodyInputChanged(String),
    SendRequest,
    HeaderKeyChanged(usize, String),
    HeaderValueChanged(usize, String),
    #[allow(dead_code)] // TODO: Remove this out-out warning
    RemoveHeader(usize),
    AddHeader,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct GUI {
    methods: combo_box::State<Method>,
    method_selected: Option<Method>,
    url_input: String,
    url_input_valid: bool,
    header_input: Vec<(String, String)>,
}

impl GUI {
    fn new() -> Self {
        Self {
            methods: combo_box::State::new(constants::METHODS.into()),
            method_selected: Some(Method::GET),
            url_input: String::new(),
            url_input_valid: false,
            header_input: vec![(String::new(), String::new())],
            // body_input: String::new(),
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
            } // Message::HeaderInputChanged(header) => {
            //     self.header_input = header; // ttp::HeaderMap::new();
            // }
            // Message::BodyInputChanged(body) => {
            //     self.body_input = body; // http::Body::from("Body");
            // }
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
            _ => {}
        }
    }

    fn view(&self) -> Element<Message> {
        // ROW: Method, URI, Send Button
        let request_row = self.view_request();

        // ROW: Headers
        let headers_column = self.view_request_headers();

        column![
            request_row,
            container(headers_column).width(Length::Fill).padding(10)
        ]
        .into()
    }

    fn view_request(&self) -> Element<Message> {
        let url_input_icon = iced::widget::text_input::Icon {
            font: iced::Font::default(),
            code_point: if self.url_input_valid { '✅' } else { '❌' },
            size: Some(default_styles::input_size()),
            spacing: 0.0,
            side: iced::widget::text_input::Side::Right,
        };

        let url_input = TextInput::new("Enter URI", &self.url_input)
            .on_input(Message::UrlInputChanged)
            .size(default_styles::input_size())
            .icon(url_input_icon)
            .width(Length::Fill);
        let method_combo_box = combo_box(
            &self.methods,
            "Method",
            self.method_selected.as_ref(),
            Message::MethodChanged,
        )
        .width(75)
        .size(default_styles::input_size_as_f32());

        let send_button = Button::new(Text::new("Send").size(20)).on_press(Message::SendRequest);
        let request_row = row![method_combo_box, url_input, send_button]
            .spacing(10)
            .padding(10)
            .align_y(iced::Alignment::Center);

        request_row.into()
    }

    fn view_request_headers(&self) -> Element<Message> {
        let headers_title = Text::new("Headers").size(16);
        let mut headers_column = column![headers_title].spacing(10);

        for (i, header) in self.header_input.iter().enumerate() {
            let header_row = row![
                TextInput::new("Key", &header.0)
                    .on_input(move |key| Message::HeaderKeyChanged(i, key))
                    .width(Length::FillPortion(1)),
                TextInput::new("Value", &header.1) // TODO: Change unwrap
                    .on_input(move |value| Message::HeaderValueChanged(i, value))
                    .width(Length::FillPortion(2)),
                Button::new(Text::new("X"))
                    .on_press(Message::RemoveHeader(i))
                    .style(button::danger)
            ]
            .spacing(10);

            headers_column = headers_column.push(header_row);
        }

        let add_header_button =
            Button::new(Text::new("Add Header").size(20)).on_press(Message::AddHeader);
        headers_column = headers_column.push(add_header_button);

        headers_column.into()
    }
}

impl Default for GUI {
    fn default() -> Self {
        GUI::new()
    }
}

mod validators {
    use crate::core::requests::Url;

    pub fn is_valid_url(url: &str) -> bool {
        Url::parse(url).is_ok()
    }
}

mod default_styles {
    pub const fn input_size_as_f32() -> f32 {
        20.0
    }

    pub const fn input_size() -> iced::Pixels {
        iced::Pixels(input_size_as_f32())
    }
}
