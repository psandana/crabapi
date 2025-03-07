use super::GUI;
use crate::gui::iced::{Message, default_styles};
use iced::widget::{Text, column, container, scrollable, text_editor};
use iced::{Center, Element, Length};
use iced_highlighter::Highlighter;

impl GUI {
    pub fn view_response(&self) -> Element<'_, Message> {
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
      column![label, scrollable(body)]
          .spacing(default_styles::spacing())
          .into()
  }
}