mod default_styles;
mod file;
mod views;

use crate::core::requests;
use crate::core::requests::{Method, constants, send_requests, validators};
use http::{HeaderMap, HeaderName};
use iced;
use iced::widget::text_editor::{Action, Content};
use iced::widget::text_editor;
use iced::widget::{column};
use iced::{Element, Task};
use reqwest::{Body, Client};
use std::path::PathBuf;
use std::sync::Arc;


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
    BodyTypeChanged(BodyType),
    BodyContentChanged(text_editor::Action),
    BodyContentOpenFile,
    BodyContentFileOpened(Result<(PathBuf, Arc<String>), file::FileOpenDialogError>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BodyType {
    Empty,
    File,
    Text,
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
    body_content: text_editor::Content,
    body_type_select: Option<BodyType>,
    body_file_path: Option<PathBuf>,
    body_file_content: Option<Arc<String>>,
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
            body_content: text_editor::Content::default(),
            body_type_select: Some(BodyType::Text),
            body_file_path: None,
            body_file_content: None,
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
            Message::BodyTypeChanged(body_type) => {
                self.body_type_select = Some(body_type);
                Task::none()
            }
            Message::BodyContentChanged(action) => {
                self.body_content.perform(action);
                Task::none()
            }
            Message::BodyContentOpenFile => {
                Task::perform(file::open_file(), Message::BodyContentFileOpened)
            }
            Message::BodyContentFileOpened(result) => {
                match result {
                    Ok((path, content)) => {
                        self.body_file_content = Some(content);
                        self.body_file_path = Some(path);
                    }
                    Err(error) => {
                        // TODO: use tracing
                        println!("Error opening file: {:?}", error);
                        if let file::FileOpenDialogError::IoError(kind) = error {
                            println!("Error kind: {:?}", kind);
                        }
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
        let headers_row = self.view_request_headers();

        // ROW: Body
        let body_row = self.view_request_body();

        // ROW: Queries
        let queries_column = self.view_request_queries();

        // ROW: Response
        let response_row = self.view_response();

        column![
            request_row,
            headers_row,
            body_row,
            queries_column,
            response_row
        ]
        .into()
    }

}

impl Default for GUI {
    fn default() -> Self {
        GUI::new()
    }
}
