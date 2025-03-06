use iced;
use iced::Element;
use iced::widget::{Column, column, combo_box, text, text_input};

use crate::core::requests::{Method, Url, constants};

pub fn init() {
    iced::run(GUI::title, GUI::update, GUI::view).unwrap()
}

#[derive(Debug, Clone)]
enum Message {
    MethodChanged(Method),
    UrlInputChanged(String),
    // HeaderInputChanged(String),
    // BodyInputChanged(String),
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct GUI {
    methods: combo_box::State<Method>,
    method_selected: Option<Method>,
    url_input: String, //http::Uri,
                       // header_input: String, //http::HeaderMap,
                       // body_input: String,   //http::Body
}

impl GUI {
    fn new() -> Self {
        Self {
            methods: combo_box::State::new(constants::METHODS.into()),
            method_selected: None,
            url_input: String::new(),
            // header_input: String::new(),
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
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            GUI::label("Uri:"),
            text_input("Uri", &self.url_input).on_input(Message::UrlInputChanged),
            GUI::label("Method:"),
            combo_box(
                &self.methods,
                "Method",
                self.method_selected.as_ref(),
                Message::MethodChanged
            )
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
