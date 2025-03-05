#[cfg(feature = "iced")]
mod iced;

pub fn run_gui() {
    #[cfg(feature = "iced")]
    {
        iced::init();
    }
}
