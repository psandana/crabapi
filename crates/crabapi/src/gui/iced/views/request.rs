use super::GUI;
use crate::gui::iced::{Message,  default_styles};
use iced::widget::{Button, Row, Text, TextInput, column, pick_list, row};
use iced::{Alignment, Element, Length};

impl GUI {
    pub fn view_request(&self) -> Element<Message> {
      let title_row = Self::view_request_row_setup(row![Self::view_request_title()]);

      let url_input = self.view_request_url_input();

      let method_input = self.view_request_method_input();

      let send_button = Self::view_request_send_button();

      let request_row = Self::view_request_row_setup(row![method_input, url_input, send_button]);

      column![title_row, request_row].into()
  }

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
}