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
    url_input: String,
    url_input_valid: bool, //http::Uri,
                           // header_input: String, //http::HeaderMap,
                           // body_input: String,   //http::Body
}

impl GUI {
    fn new() -> Self {
        Self {
            methods: combo_box::State::new(constants::METHODS.into()),
            method_selected: None,
            url_input: String::new(),
            url_input_valid: false,
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
                self.url_input = url;
                self.url_input_valid = GUI::is_valid_url(&self.url_input);
            } // Message::HeaderInputChanged(header) => {
              //     self.header_input = header; // ttp::HeaderMap::new();
              // }
              // Message::BodyInputChanged(body) => {
              //     self.body_input = body; // http::Body::from("Body");
              // }
        }
    }

    fn view(&self) -> Element<Message> {
        let url_input_icon = iced::widget::text_input::Icon {
            font: iced::Font::default(),
            code_point: if self.url_input_valid { '✅' } else { '❌' },
            size: Some(Self::input_size()),
            spacing: 0.0,
            side: iced::widget::text_input::Side::Right,
        };
        let url_input = text_input("Url", &self.url_input)
            .on_input(Message::UrlInputChanged)
            // .padding(10)
            .size(Self::input_size())
            .icon(url_input_icon);

        column![
            GUI::label("Url:"),
            url_input,
            GUI::label("Method:"),
            combo_box(
                &self.methods,
                "Method",
                self.method_selected.as_ref(),
                Message::MethodChanged
            )
            .size(Self::input_size_as_f32())
        ]
        .into()
    }

    fn label(label: &str) -> Column<'_, Message> {
        column![text(label)].spacing(10)
    }

    fn is_valid_url(url: &str) -> bool {
        Url::parse(url).is_ok()
    }

    const fn input_size_as_f32() -> f32 {
        20.0
    }

    const fn input_size() -> iced::Pixels {
        iced::Pixels(Self::input_size_as_f32())
    }
}

impl Default for GUI {
    fn default() -> Self {
        GUI::new()
    }
}
