use crate::app::*;
use crate::components::*;
use crate::utils::*;
use iced::widget::svg;
use iced::{Element, Length};

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub struct Shortcut {
    name: String,
    icon: &'static [u8], // PathBuf
    action: Option<String>,
}

impl Shortcut {
    pub fn new(name: String, icon: &'static [u8], action: Option<String>) -> Self {
        Shortcut { name, icon, action }
    }

    fn icon(&self) -> Element<main_app::Message> {
        let icon = svg_path::set_fill(self.icon, String::from("White"));

        svg(svg::Handle::from_memory(icon))
            .width(Length::Fixed(80.))
            .height(Length::Fixed(80.))
            .into()
    }

    pub fn view(&self, index: usize) -> Element<main_app::Message> {
        Key::new(self.icon())
            .on_press(main_app::Message::ViewHandler(view::Message::ViewMessage(
                index,
            )))
            .into()
    }
}
