#[allow(unused_imports)]
use iced::{Color, Pixels, Border, Padding, border::Radius};

/// Font size for inputs and buttons
pub const fn input_size_as_f32() -> f32 {
    22.0  // Slightly larger for better readability
}

/// Font size wrapper
pub const fn input_size() -> Pixels {
    Pixels(input_size_as_f32())
}

/// Padding for UI elements
pub const fn padding() -> Padding {
    Padding::new(12.0)  // Increased padding for better spacing
}

/// Spacing between elements
pub const fn spacing() -> Pixels {
    Pixels(12.0)
}
