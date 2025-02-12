use std::path::PathBuf;
use iced::widget::{image, 
    svg
};
use iced::{Element, 
    Length
};
use crate::components::*;
use crate::app::*;
use crate::utils::*;


#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub struct Shortcut {
    name: String,
    icon: Option<PathBuf>,
    action: Option<String>,
}


impl Shortcut {
    pub fn new(name: String, icon: Option<PathBuf>, action: Option<String>) -> Self {
        Shortcut {
            name,
            icon,
            action,
        }
    }

    fn icon(&self) -> Element<main_app::Message> {
        match &self.icon {
            Some(path) => {
                if path
                    .as_os_str()
                    .to_str()
                    .is_some_and(|pathname| pathname.ends_with("png"))
                {
                    image(image::Handle::from_path(path))
                        .width(Length::Fixed(80.))
                        .height(Length::Fixed(80.))
                        .into()
                } else {
                    svg(svg::Handle::from_path(path))
                        .width(Length::Fixed(80.))
                        .height(Length::Fixed(80.))
                        .into()
                }
            }
            None => svg(svg::Handle::from_memory(globals::DEFAULT_ICON))
                .width(Length::Fixed(80.))
                .height(Length::Fixed(80.))
                .into(),
        }
    }

    pub fn view(&self, index: usize) -> Element<main_app::Message> {
        Key::new(self.icon())
            .on_press(main_app::Message::ViewHandler(view::Message::ViewMessage(index)))
            .into()
    }
}