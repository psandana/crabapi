use iced;
use iced::widget::{Button, Column, Text, TextInput, column, row, text};
use iced::widget::{button, combo_box, container};
use iced::{Element, Length};

use http::Method;

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
    RemoveHeader(usize),
    AddHeader,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct GUI {
    methods: combo_box::State<Method>,
    method_selected: Option<Method>,
    url_input: String, //http::Uri,
    header_input: Vec<(String, String)>, //http::HeaderMap,
                       // body_input: String,   //http::Body
}

impl GUI {
    fn new() -> Self {
        Self {
            methods: combo_box::State::new(vec![
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::PATCH,
            ]),
            method_selected: None,
            url_input: String::new(),
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
                self.url_input = url; //http::Uri::from_static("http://localhost:7878");
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
        let url_input = TextInput::new("Enter URI", &self.url_input)
            .on_input(Message::UrlInputChanged)
            .width(Length::Fill);
        let method_combo_box = combo_box(
            &self.methods,
            "Method",
            self.method_selected.as_ref(),
            Message::MethodChanged,
        )
        .width(75);
        let send_button = Button::new(Text::new("Send").size(20)).on_press(Message::SendRequest);
        let request_row = row![method_combo_box, url_input, send_button]
            .spacing(10)
            .padding(10)
            .align_y(iced::Alignment::Center);

        // ROW: Headers
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

        column![
            request_row,
            container(headers_column).width(Length::Fill).padding(10)
        ]
        .into()
    }

    fn label(label: &str) -> Column<'_, Message> {
        column![text(label)].spacing(10)
    }
}

impl Default for GUI {
    fn default() -> Self {
        GUI::new()
    }
}
