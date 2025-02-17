use crate::app::*;
use crate::components::*;
use crate::utils::*;
use iced::widget::svg;
use iced::{Element, Length};

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub struct Shortcut {
    pub name: String,
    pub icon: &'static [u8],
    pub action: Option<String>,
}

impl Shortcut {
    pub fn new(name: String, icon: &'static [u8], action: Option<String>) -> Self {
        let icon = functions::set_svg_fill(icon, String::from("White"));
        Shortcut { name, icon, action }
    }

    fn icon(&self) -> Element<main_app::Message> {
        svg(svg::Handle::from_memory(self.icon))
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
