pub const fn input_size_as_f32() -> f32 {
    20.0
}

pub const fn input_size() -> iced::Pixels {
    iced::Pixels(input_size_as_f32())
}

pub const fn padding() -> iced::Padding {
    iced::Padding::new(10.0)
}

pub const fn spacing() -> iced::Pixels {
    iced::Pixels(10.0)
}
